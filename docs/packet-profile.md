# Packet bit geometry

The inherited Packet dispatch uses flowchart placement; ranges survive only as
labels. P40 adds typed PacketField start/end/label records and a dedicated 32-bit
row layout. Fields crossing a row split into adjacent labelled segments; field
width represents bit count. Explicit bit endpoints are shown. No flowchart edges
or implicit scheduling. Input is bounded to 4096 bits and 128 fields, no overlap.
The initial profile accepts explicit ranges and +count, not init configuration.
SVG remains the presentation format. Measured wrapped labels determine row height.
