# Packet bit geometry

The inherited Packet dispatch uses flowchart placement; ranges survive only as
labels. P40 adds typed PacketField start/end/label records and a dedicated 32-bit
row layout. Fields crossing a row split into adjacent labelled segments; field
width represents bit count. Explicit bit endpoints are shown. No flowchart edges
or implicit scheduling. Input is bounded to 4096 bits and 128 fields, no overlap.
The initial profile accepts explicit ranges and +count, not init configuration.
SVG remains the presentation format. Measured wrapped labels determine row height.

## XFMD N14 readability correction

Bit pitch is shared across the diagram and reserves the actual rendered font
width of endpoint numbers and bracketed ordinal markers. Endpoints are centered
on their first/last bit cells, with only one number for a single-bit field.
Field widths remain proportional. Wrapped names that still exceed a segment's
padded width become `[n]`, with a full per-row legend containing that ordinal,
segment bit range and name. Row-spanning fields retain their ordinal. Legend
text expands the scene if an unbreakable word cannot wrap; it is never clipped
inside the bit cell. The normal SVG path serves both preview and PDF.
