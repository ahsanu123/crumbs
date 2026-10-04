<img  src="./realistic-crumbs-logo.png" style="height: 250px; " align="right"/> 
 
### CRUMBS
- **Hardware Status** ✔️ Production Done
- **Hardware Status** 🏁 Prototype Done
- **Software Status** ☑️ Finished
- **Software Status** ⭐ POC only
- ⏳ In Progress

| Name                                                     | Hardware Status  | Software Status | Enclosure       | 
|----------------------------------------------------------|------------------|-----------------|-----------------|
|  TMD - Temperature Measurement Dongle                    |       🏁         |       ⭐        |     ⏳          |


## UV Laser and PCB Making 

simple direct step to generate quite accurate 2 layer PCB with only paper, pcb board, uv laser and free software 

- make sure to add corner dot marker in bottom and front layer, so bottom layer will correclty flipped during flatcam process
- generate gerber from eda software
- import it into flatcam, add buffer `0.06` to all copper layer (front and bottom)
- do isolation routing to all copper layer, make sure to flip for bottom layer before do isolation routing
- choose pre-processor `grbl laser`
- generate `nc` file to use for uv routing
- open `laser grbl`, choose _center corner marker_ to marking pcb center point, and help when flipping pcb for bottom layer routing.
- after top and bottom route complete, do drill on corner marker to make sure front and bottom is in correct position
- do etching
- then drill all component hole.


## References 

- [ESP32 S3 Series IO MUX Map](https://documentation.espressif.com/esp32-s3_datasheet_en.pdf)
- [Emi's Reverse Engineer HCSR04](https://uglyduck.vajn.icu/ep/archive/2014/01/Making_a_better_HC_SR04_Echo_Locator.html)

<sub>a long project with the hope of completion. created with ♥️ by ah...</sub>
