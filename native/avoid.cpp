// Synchronous C ABI. All libavoid objects are owned by the local Router.
#include "libavoid/libavoid.h"
#include <vector>
#include <algorithm>
#include <map>
#include <stdexcept>
#include <cstdio>
#include <cstdint>
struct Pt { double x, y; };
struct Pin { Pt p; uint32_t dirs; };
struct Shape { uint32_t id, first, count, first_pin, pin_count; };
struct Edge { uint32_t id, source, target, source_dirs, target_dirs; Pt source_pin, target_pin; uint32_t locked; };
using Continue = uint8_t (*)(void*);
using Emit = uint8_t (*)(void*, uint32_t, const Pt*, size_t);
class TransactionRouter final : public Avoid::Router {
  Continue check_; void* context_;
public:
  bool aborted = false;
  TransactionRouter(Continue check, void* context) : Router(Avoid::OrthogonalRouting), check_(check), context_(context) {}
  bool shouldContinueTransactionWithProgress(unsigned, unsigned, unsigned, double) override {
    if (!check_(context_)) aborted = true;
    return !aborted;
  }
};
extern "C" int mermaid_avoid_route(const Pt* points, const Pin* pins,
    const Shape* shapes, size_t shape_count, const Edge* edges, size_t edge_count,
    double clearance, double separation, double bend_cost,
    Continue check, Emit emit, void* context, char* error, size_t error_size) noexcept {
  try {
    if (!check(context)) return 1;
    TransactionRouter router(check, context);
    router.setTransactionUse(true);
    router.setRoutingParameter(Avoid::segmentPenalty, bend_cost);
    router.setRoutingParameter(Avoid::crossingPenalty, 0);
    router.setRoutingParameter(Avoid::fixedSharedPathPenalty, 0);
    router.setRoutingParameter(Avoid::shapeBufferDistance, clearance);
    router.setRoutingParameter(Avoid::idealNudgingDistance, separation);
    router.setRoutingOption(Avoid::nudgeOrthogonalSegmentsConnectedToShapes, false);
    router.setRoutingOption(Avoid::nudgeSharedPathsWithCommonEndPoint, true);
    router.setRoutingOption(Avoid::nudgeOrthogonalTouchingColinearSegments, true);
    std::map<uint32_t, Avoid::ShapeRef*> refs;
    for (size_t i=0; i<shape_count; ++i) {
      if (!check(context)) return 1;
      const auto& s = shapes[i];
      Avoid::Polygon poly(s.count);
      for (size_t j=0;j<s.count;++j) poly.ps[j] = Avoid::Point(points[s.first+j].x, points[s.first+j].y);
      auto* shape = new Avoid::ShapeRef(&router, poly, unsigned(i+1));
      refs.emplace(s.id, shape);
      const auto bounds = shape->polygon().offsetBoundingBox(0);
      // Pin classes are direction masks. Libavoid chooses the pin within a class.
      for (unsigned mask=1;mask<=15;++mask) {
        bool used = false;
        for (size_t k=0;k<edge_count;++k) {
          used |= (!(edges[k].locked&1) && edges[k].source==s.id && edges[k].source_dirs==mask) ||
                  (!(edges[k].locked&2) && edges[k].target==s.id && edges[k].target_dirs==mask);
        }
        if (!used) continue;
        for (size_t j=0;j<s.pin_count;++j) {
          const auto& p = pins[s.first_pin+j];
          if (!(p.dirs & mask)) continue;
          auto* pin = new Avoid::ShapeConnectionPin(shape, mask,
              std::clamp(p.p.x - bounds.min.x, 0.0, bounds.max.x-bounds.min.x),
              std::clamp(p.p.y - bounds.min.y, 0.0, bounds.max.y-bounds.min.y), false, 0, p.dirs & mask);
          pin->setExclusive(true);
        }
      }
    }
    std::vector<Avoid::ConnRef*> connectors;
    for (size_t i=0;i<edge_count;++i) {
      const auto& e = edges[i];
      auto fixedPin = [&](bool source) {
        const unsigned bit=source ? 1 : 2;
        const unsigned dirs=source ? e.source_dirs : e.target_dirs;
        if (!(e.locked & bit)) return dirs;
        const auto p=source ? e.source_pin : e.target_pin;
        auto* shape=refs.at(source ? e.source : e.target);
        const auto bounds=shape->polygon().offsetBoundingBox(0);
        const unsigned cls=16+2*i+(source?0:1);
        auto* pin=new Avoid::ShapeConnectionPin(shape,cls,
          std::clamp(p.x-bounds.min.x,0.0,bounds.max.x-bounds.min.x),
          std::clamp(p.y-bounds.min.y,0.0,bounds.max.y-bounds.min.y),false,0,dirs);
        pin->setExclusive(true); return cls;
      };
      auto* conn = new Avoid::ConnRef(&router,
          Avoid::ConnEnd(refs.at(e.source), fixedPin(true)),
          Avoid::ConnEnd(refs.at(e.target), fixedPin(false)), unsigned(shape_count+i+1));
      conn->setRoutingType(Avoid::ConnType_Orthogonal);
      connectors.push_back(conn);
    }
    router.processTransaction();
    if (router.aborted || !check(context)) return 1;
    for (size_t i=0;i<edge_count;++i) {
      const auto& route = connectors[i]->displayRoute();
      std::vector<Pt> result;
      for (const auto& p : route.ps) result.push_back({p.x,p.y});
      if (result.size()<2) throw std::runtime_error("libavoid returned no route");
      if (!emit(context,edges[i].id,result.data(),result.size())) return 2;
    }
    return 0;
  } catch (const std::exception& e) {
    if (error_size) std::snprintf(error,error_size,"%s",e.what());
  } catch (...) {
    if (error_size) std::snprintf(error,error_size,"unknown C++ exception");
  }
  return 2;
}
