use super::*;
/// Fixed bit grid; text influences row height, never the encoded field width.
pub(super) fn compute(graph:&Graph,theme:&Theme,config:&LayoutConfig)->Layout{
 let mut nodes=BTreeMap::new();let rows=graph.packet_fields.iter().map(|f|f.end/32+1).max().unwrap_or(1);
 let bit=26.;let mut y=24.;
 for row in 0..rows {measurements::checkpoint();let mut cells=vec![];let mut height=48f32;
  for (index,f) in graph.packet_fields.iter().enumerate(){let start=f.start.max(row*32);let end=f.end.min(row*32+31);if start>end{continue;}
   let width=(end-start+1) as f32*bit;let block=measure_label_with_max_width(&f.label,theme.font_size,(width-10.).max(8.),config,true,theme.font_family.as_str());height=height.max(block.height+16.);cells.push((index,start,end,width,block));
  }
  for (index,start,end,width,label) in cells {let x=24.+(start%32) as f32*bit;
   let id=format!("packet_{index}_{row}");nodes.insert(id.clone(),NodeLayout{id,x,y:y+20.,width,height,label,shape:crate::ir::NodeShape::Rectangle,style:Default::default(),link:None,anchor_subgraph:None,hidden:false,icon:None});
   for (suffix,value,x) in [("start",start,x),("end",end,x+width-20.)] {if suffix=="end"&&start==end{continue;}let id=format!("bit_{index}_{row}_{suffix}");let label=measure_label_with_font_size(&value.to_string(),theme.font_size*0.7,config,false,theme.font_family.as_str());nodes.insert(id.clone(),NodeLayout{id,x,y,width:20.,height:18.,label,shape:crate::ir::NodeShape::Text,style:Default::default(),link:None,anchor_subgraph:None,hidden:false,icon:None});}
  }y+=height+40.;
 }
 Layout{kind:graph.kind,nodes,edges:vec![],subgraphs:vec![],width:32.*bit+48.,height:y,diagram:DiagramData::Graph{state_notes:vec![]}}
}
