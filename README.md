# BrainCube Power Handler
BrainCube will be running up to 16 Raspberry Pi nodes. Need the ability to safely turn on and more importantly off. 
The nodes will be configured to power up or down when a GPIO pins is pulled low. I'll need to make sure that I can 
drive all the R.Pi gpios from the same Pico GPIO. 

I also want to be able to monitor the load that each node is under, and provide a pretty sci-fi inspired
UI on the round TFT display that I have. Use the power button that is already on the case for toggling the power
on and off.

I've already written a little Rust application that reads the Linux process load data file and out puts a PWM 
signal on a GPIO pin. 

## Requirements
(Listed in implementation order)
- Read a momentary push button
- Toggle the state of an output pin
- Measure the pulse length of PWM pulses on 16 input pins
- Configure SPI and misc pins for the round TFT
- Initialise the round TFT
- Draw a test pattern on the TFT
- Design the UI
- Implement the UI as an RLE image. Colours are used as indexes into a palette, display colour is mapped at run time to the load value / image 
