# Single Side PCB Manufacture Documentation With FlatCam , LightBurn and AtomStack A12 Pro

## FlatCam

- export kicad project to gerber, also generate drill as gerber 
- open flatcam, import F_Cu (front copper), and PTH drill 
- use substract tool to substract F_Cu with PTH drill, change name to `Copper`
- next, import F_Paste (front paste), and PTH drill
- select F_Paste , Mask and PTH drill then merge it with, Edit -> Join Object -> Join Gerber. delete join source, and change name to `Front Paste`
- double click `Copper` on left panel, then select `gerber editor`, select all Copper (drag and select) use buffer tool to add buffer with value (buffer distance) of `0.06`
- click exit editor, and hit yes on save confirmation
- you can remove old `Copper` and rename `Copper_edit` to `Copper`
- do same thing to `Front Paste`, add buffer (but make buffer have more value, overlap is ok) and delete source, then rename edited into `Front Copper`
- next, double click on `Copper` then choose isolation routing, 
- next step you can play with value if result is not good
- change diameter value on tools table, to `0.06` 
- change `Tool Dia` to `0.06`
- change `Passes` to `20`
- change `Overlap` to `33%`
- then click `Generate Geometry`
- `Geometry Object` left panel will opened
- choose `preprocessor` to `GRBL_Laser`
- change `V-Tip Dia` to `0.06`
- change `Laser Power` to `5000`
- then click `Generate CNCJob object` button
- click `Save CNC Code` button, and save file somewhere, name it `Copper_NC`
- you can hide generate CNC Job and Geometry to give better performance (sometimes its laggy because of that)
- next, select `Fron Paste` then use `Paint Tool` from `Tool` dropdown
- again, change `Diameter` on `Tools Table` to `0.06`
- change `method` to `lines` and `overlap` to `33%`
- click on `Generate Geometry` Button
- select on `Generate Geometry` then do same think as `Copper_NC`, diameter to `0.06`, `preprocessor` to `GRBL_Laser` and `Laser Power` to `5000`
- then, click `Generate CNCJob object` button
- click `Save CNC Code` button, and save file somewhere, name it `Front Paste_NC`

## LightBurn

- next open LightBurn 
- import `Copper_NC` and `Front Paste_NC` make it difference layer, by select and click on layer (numbered small square with color in bottom ui)
- for `Copper_NC` we will use `55%` power with `100%` speed
- for `Front Paste_NC` we will use `35%` power with `100%` speed, 6-10 times
- play with that number if result not good

