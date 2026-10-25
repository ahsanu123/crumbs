## UV Laser and PCB Making 

<img width="656" height="627" alt="image" src="https://github.com/user-attachments/assets/85cc944e-ded5-48b9-b30c-3f120e408d0b" />

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

<img width="935" height="674" alt="image" src="https://github.com/user-attachments/assets/b03507a7-5ce0-4fc4-8bd2-6019a5b17c8d" />

