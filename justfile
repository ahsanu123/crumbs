# For symbol + footprint + 3d model (with --full argument)
# easyeda2kicad --full --lcsc_id=C2040
e2kfull lcsc_id: 
  easyeda2kicad --full --lcsc_id={{lcsc_id}} --output ./libraries

esvg fileName: 
  kicad-cli pcb export svg --mode-single --layers F.Cu,B.Cu,F.Silkscreen,B.Silkscreen,F.Mask,B.Mask,Edge.Cuts {{fileName}}
