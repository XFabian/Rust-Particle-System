# Rust-Particle-System

Create small circles that have collision detection. Alpha value is cahnged each frame. If two circles have a small distance a line is added.
The alpha value of the line corresponds to the distance between the two circles. It looks pretty nice :D

As dependency only glium is used for drawing. 

Run the program with cargo run <number_of_circles>.

While the program is running and the window is focused you can add and remove circles with the N/R key on your keyboard respectively.
You can press Esc to exit the program.


This was created to learn Rust. Since the glium Tutorial only renders one triangle you can use this to look at an example of multiple moving objects that
are rendered to the screen. We take advantage of instancing. We jsust create one object and create coordinates per instance and apply 
transformations in the shader. This makes the code pretty fast and you can draw a bunch of circles.

TODO
Add spatial hash so we dont have to compare all circles against each other if we check for collisions
