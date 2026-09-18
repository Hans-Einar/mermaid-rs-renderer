# BX-HOST/0.1-draft1 contract artifacts

Schemas define the owner-accepted draft1 authoring payload, typed model, prepare
request, prepared frame, intent and result. The phase-046 Rust implementation
conforms to the renderer subset; C++/FFI integration is separately verified.
The fixture frame in `fixtures/frame.mock.json` remains hand-authored. Real
generated frames are checked by `check_boxui_runtime.py`. Source JSON follows
the `boxui 0.1` header; it is not executable SDL.

Schema checks cover shape and field types. The host contract additionally requires
unique IDs, valid binding roles/types, min<=max, aggregate byte/depth limits,
finite coordinates, rect/clip containment, uint64 range, enabled-state authority,
source/frame equality and safe SVG. An XML parser accepting SVG proves none of
those obligations. Unknown fields reject in this profile; future extensions need
a new version. String/number/boolean values match the binding type without coercion.

Input control commandBinding carries the event bindingId; valueBinding identifies
the editable value. They have independent revisions. Inputs are string-only in
0.1; buttons take `none`; read-only values can be string/number/boolean. `none`
is only a command argument type. The full host key scopes the state snapshot.

Fixtures under ../../09--Verification/fixtures include a Markdown source example,
mock frame, typed intent and negative variants. Mock sizes are not visual evidence.

Normalized model replaces embedded source with childRef. Child interpretation belongs
to XFMD interpreter; application prepares typed children before parent layout.
