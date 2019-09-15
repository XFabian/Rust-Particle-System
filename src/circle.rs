use std::convert::TryInto;
use crate::vec2::Vec2;
use rand::{thread_rng, Rng};
use glium::VertexBuffer;
use glium;
pub const SCREEN_WIDTH: f32 = 1.0; // coordinate system goes from -1 to 1
pub const SCREEN_HEIGHT: f32 = 1.0;
pub const LINE_THRESHOLD: f32 = 0.3; // 0.3 was before
pub const RADIUS: f32 = 0.01;
const NUM_SEGMENTS: u32 = 30;
const ALPHA_STEP_SIZE: f32 = 0.005;


#[derive(Copy, Clone, Debug)]
    pub struct Vertex {
        pub position: [f32; 2], // Holds coordinates for the triangles
    }

implement_vertex!(Vertex, position);



#[derive(PartialEq, Debug)] // PartialEq can compare with other Circles
pub struct Circle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub alpha: f32,
    alpha_step_size:f32,
}

impl Circle {

    pub fn new(pos_x: f32, pos_y: f32, radius: f32, dir_x: f32, dir_y: f32) -> Circle{
        Circle{position: Vec2::new(pos_x, pos_y),
               velocity: Vec2::new(dir_x, dir_y),
               radius: radius,
               alpha: thread_rng().gen_range(0.1, 1.0),
               alpha_step_size: ALPHA_STEP_SIZE,
               }
    }


    pub fn get_collision_point(circle1: Circle, circle2: Circle) -> Vec2 {
        // greta insight https://stackoverflow.com/questions/1736734/circle-circle-collision
        let diff_vec: Vec2 = circle2.position - circle1.position; // need this in absolute
        diff_vec
    }


    pub fn create_circle(&self) -> Vec<Vertex> {
    
        let mut shape: Vec<Vertex> = Vec::new();
        for i in 0..NUM_SEGMENTS {
            let theta = 2.0 * 3.1415926 * (i as f32) / (NUM_SEGMENTS as f32); // current angle last argument makes the circle better but more computatuons

            let x = self.radius * theta.cos();
            let y = self.radius * theta.sin();
            let v1 = Vertex { position: [self.position.x + x, self.position.y + y]};
            shape.push(v1);
        }
        //let vertex_buffer = glium::VertexBuffer::dynamic(display, &shape).unwrap();
        shape
        /* TODO This could be better http://slabode.exofire.net/circle_draw.shtml
        float theta = 2 * 3.1415926 / float(num_segments); 
	float tangetial_factor = tanf(theta);//calculate the tangential factor 

	float radial_factor = cosf(theta);//calculate the radial factor 
	
	float x = r;//we start at angle = 0 

	float y = 0; 
    
	glBegin(GL_LINE_LOOP); 
	for(int ii = 0; ii < num_segments; ii++) 
	{ 
		glVertex2f(x + cx, y + cy);//output vertex 
        
		//calculate the tangential vector 
		//remember, the radial vector is (x, y) 
		//to get the tangential vector we flip those coordinates and negate one of them 

		float tx = -y; 
		float ty = x; 
        
		//add the tangential vector 

		x += tx * tangetial_factor; 
		y += ty * tangetial_factor; 
        
		//correct using the radial factor 

		x *= radial_factor; 
		y *= radial_factor; 
	} 
	glEnd();
    */
    }
    
    // Mass of an object right now is only the Radius https://stackoverflow.com/questions/345838/ball-to-ball-collision-detection-and-handling
    // Atleast direction is ok but momentum is not that good 
    pub fn resolveCollision(circle1: &mut Circle, circle2: &mut Circle){

        let mut diff_centre_norm = (circle1.position - circle2.position).normalize();
        // TODO http://geekswithblogs.net/robp/archive/2008/05/15/adding-collision-detection.aspx
        let vel1 = circle1.velocity;

        let x1: f32 = vel1.dot(diff_centre_norm);

        let v1x = diff_centre_norm * x1;
        let v1y = vel1 - v1x;

        diff_centre_norm = - diff_centre_norm; // for the other direction
        let vel2 = circle2.velocity;
        let x2: f32 = diff_centre_norm.dot(vel2);

        let v2x = diff_centre_norm * x2;
        let v2y = vel2 - v2x;


        let m1 = circle1.radius;
        let m2 = circle2.radius;
        let combined_mass = m1 + m2;
        /* best to check with a circle with 0 velocity let cir = Circle::new(11.0,11.0,RADIUS, 0.0, 0.0); With the old claculation one would only exchange speed.
         so the other stops
        let new_vel_1 = (circle1.velocity * (RADIUS - RADIUS) + (circle2.velocity * 2.0 * RADIUS)) *  (1.0 / (RADIUS + RADIUS));
        let new_vel_2 = (circle2.velocity * (RADIUS - RADIUS) + (circle1.velocity * 2.0 * RADIUS)) * (1.0 / (RADIUS + RADIUS));
        */
        let new_vel_a = (v1x * ((m1 - m2) / combined_mass)) + (v2x *((2.0 * m2) / combined_mass)) + v1y;
        let new_vel_b = (v2x * ((m2 - m1) / combined_mass)) + (v1x * ((2.0 * m1) / combined_mass)) + v2y;

        
        
        //println!("New x {}, new y {}", newVelX1, newVelY1);
        //println!("New x {}, new y {}", newVelX2, newVelY2);
        circle1.velocity = new_vel_a;
        circle1.move_circle();
        // We need to move both circles so that they are not stuck together first collision will reverse directins and second will reverse again causing them to move together
        // another explanatin is at then of this post : https://gamedevelopment.tutsplus.com/tutorials/when-worlds-collide-simulating-circle-circle-collisions--gamedev-769
        circle2.velocity = new_vel_b;
        circle2.move_circle();
    }

    pub fn move_circle(&mut self) {
        self.position += self.velocity;
    }

    fn invert_direction(&mut self) {
        self.velocity = -self.velocity;
    }

    pub fn check_bounds(&mut self) {
        let radius_2d = Vec2::new(self.radius.into(), self.radius.into());
        let perimeter_add= self.position + radius_2d;
        let perimeter_sub = self.position - radius_2d;
        // Lets test we correct if we are outside the box we need to break free if we are outside
        // need 4 cases here 
        if perimeter_add.x >= SCREEN_WIDTH as f32 { // object came from left
            self.position.x = SCREEN_WIDTH - self.radius;
            self.velocity.x = -self.velocity.x;
        }
        if perimeter_sub.x <= -1.0 { // object came from right
            self.position.x = -1.0 + self.radius;
            self.velocity.x = -self.velocity.x;
        }
        /*
        if perimeter_add.x >= SCREEN_WIDTH as f32 || perimeter_sub.x <= -1.0 {
            self.velocity.x = -self.velocity.x;
        }
        */
        if perimeter_add.y >= SCREEN_HEIGHT as f32 { // from bottom
            self.position.y = SCREEN_HEIGHT - self.radius;
            self.velocity.y = -self.velocity.y;
        }
        if  perimeter_sub.y <= -1.0 { // from top
            self.position.y = -1.0 + self.radius;
            self.velocity.y = -self.velocity.y;
        }

    }
    // returns the predicted move useful to calcualte collisions
    pub fn predicted_move(&self) -> Vec2 {
        self.position + self.velocity
    }

    // Check if circle intersects some other circle that is in the canvas right now. Before or after they move? => After a possible move we check 
    pub fn check_intersect(&self, other: &Circle) -> bool{
            let diff = self.position - other.position;

            let distance = diff.squared_length();
            //println!("Squared Distance {}", distance);
            let squared_radius = ((self.radius + other.radius) as f32).powi(2);
            // Now check if the distance is smaller than one of the radii. Then there is an intersection
            if distance < squared_radius{ // As optimization we look at the squared radius. So we dont have to compute the square root
                return true;
            }
            else {
                return false;
            } 
    }

    // if circles a below a certain threshold. we draw a line between these two circles
    pub fn check_draw_line(&self, other: & Circle) -> (bool, f32) {
        let diff = self.position - other.position;
        let distance = diff.length();
        if distance <= LINE_THRESHOLD {
            return (true, distance);
        }
        else {
            return (false, distance);
        }
    }

    // Apply alpha step size to alpha value and return the alpha
    pub fn compute_alpha(&mut self)  {
        
        if self.alpha >= 1.0 {
            self.alpha_step_size *= -1.0;
        }
        else if self.alpha <= 0.1{
            self.alpha_step_size *=-1.0;
        }
        self.alpha += self.alpha_step_size;
    }
}

/* MOVED CODE to test flium
mod vec2;
mod circle;
use circle::{Circle, RADIUS};

extern crate sdl2;
use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect;
use rand::{thread_rng, Rng};
use std::env;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;



fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    let circle_number: i16 = args[1].parse().unwrap();

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;


canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
// fills the canvas with the color we set in `set_draw_color`.
canvas.clear();
let texture_creator = canvas.texture_creator();
// maybe apply textures to rectangles with centre point the circle and the radius. So smallest rectangle to fit the circle
let color = pixels::Color::RGB(0, 0, 255);
let color = pixels::Color::RGBA(0, 0, 255, 100);
println!("{:?}", canvas.blend_mode());

// TODO second idea. Also implement a small particle system similar to this http://slicker.me/javascript/particles.htm
// Should not be that difficuolt 
// 1. Move one circle on the screen
// 2. Abstract circles into class / trait that has bounce checks and the coordinates and check if another circle is in them
// 3. how to do the check if a circle is in another one?  
// 4. spawn a number of random circles in the beginning with random velocity. But make sure that circles dont intersect each other
// 5. Draw lines between circles if they are close together. Done
// Finished
// 6. Correct bounce behavoiur if a collision occurs Done but Mo
// 6.5 Momentum after bounce needs to change
// 7. Button to create or delete new Circles
// Make it more beatiful using surfaces? anti aliasing . Mini shadows. textures. blend mode, alpha value opaque, additive mode. So maybe create a texture for the circles and lines
// Make lines thicker when they are closer (Linear Mapping)
// When they collide small particle effect
// Also use the openGL library that can ge used gl-rs or glium for higher abstraction
// 8. I want a gradient for my circles simioar to renderer-texture.rs can also bind opengl textures
// Delete the circle that you clicked on would be best
// 10. Create or use a vector class that can be used for the circle to make certain computations easier for me
// https://gamedevelopment.tutsplus.com/tutorials/when-worlds-collide-simulating-circle-circle-collisions--gamedev-769 for some ball stuff
// http://geekswithblogs.net/robp/archive/2008/05/15/adding-collision-detection.aspx for correct collsion since actually vectors dont exchange speed
// 11. screen flickers => Why? had a present after set color that was unnecessary
// 12. Refactor code into different files 

// TODO rewrite this in Vulkano. For that I only need a method to draw my circles
// However the canvas has not been updated to the window yet,
// everything has been processed to an internal buffer,
// but if we want our buffer to be displayed on the window,
// we need to call `present`. We need to call this every time
// we want to render a new frame on the window.
canvas.present();
let mut circles = create_random_circles(circle_number);
canvas.set_blend_mode(sdl2::render::BlendMode::Add);
println!("{:?}", canvas.blend_mode());
/*
let mut circles: Vec<Circle> = Vec::new();
let cir = Circle::new(11.0,11.0,RADIUS, 0.5, 0.1);
let cir2 = Circle::new(31.0,31.0,RADIUS, 1.0, 1.0);
circles.push(cir);
circles.push(cir2); */
let mut event_pump = sdl_context.event_pump()?;
'running: loop {
    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0)); // THis is how you clear the screen
    canvas.clear();

    // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1));

    // Then we check if circles intersect and if yes we invert the direction and move
    for i in 0..circles.len() {
        let mut circle = circles.remove(0); // start at beginning and get each element. Since remove pushes all to the left we always take 0s element

        // check interseciton with rest of the circles and draw lines
        for circle_comp in &mut circles {
            if circle.check_intersect(&circle_comp) { // if they intersect both circles move ito the different direction
                //println!("Collision");
                Circle::resolveCollision(&mut circle, circle_comp);
            }
            if circle.check_draw_line(&circle_comp) {
                // if we draw the line here we lose it will be off one pixel
                canvas.line(circle.position.x as i16, circle.position.y as i16, circle_comp.position.x as i16, 
                            circle_comp.position.y as i16, color);
                let color2 = pixels::Color::RGBA(255, 0, 0,100);
                canvas.line(circle.position.x as i16, circle.position.y as i16, circle_comp.position.x as i16, 
                            circle_comp.position.y as i16, color2);
            }
        } 
        circles.push(circle); // push circle again into vector at the end

    }
    for circle in &mut circles { // check intersection with all other circles
        circle.check_bounds(); // only move the ones that are flagged when they intersected because they need to move again
        circle.move_circle();
        circle.draw_circle(&canvas);
        // since I dont care about the order
    }


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => { // New circle 
                    println!("Add random circle to screen");
                    let mut circle_new = generate_circle();
                    while check_intersections(&circle_new, &circles) {
                        println!("In while loop");
                        circle_new = generate_circle();
                    }
                    circles.push(circle_new); // Has no check right now if there is a circle right now
                    
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => { // remove random circle No error handling when there is no circle left
                    if circles.len() == 0 {continue}
                    println!("Remove random circle");
                    let mut rng = thread_rng();
                    let idx_remove = rng.gen_range(0, circles.len());
                    circles.remove(idx_remove);
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }
// present does not "clear" the buffer, that means that
// you have to clear it yourself before rendering again,
// otherwise leftovers of what you've renderer before might
// show up on the window !
//
// A good rule of thumb is to `clear()`, draw every texture
// needed, and then `present()`; repeat this every new frame.
 Ok(())
}

// Create valid circles at random positon with random velocity. The velocity is between [-3, 3] in both directions. number is the number of circles to create
fn create_random_circles(number: i16) -> Vec<Circle> {
    let mut circles: Vec<Circle> = Vec::new();
    
    let mut i = 0;
    while i < number {
        
        let circle_new = generate_circle();
        let mut intersects = check_intersections(&circle_new, &circles);
        
        if intersects {continue;}
        // we also have to check if it intersects with any other circle created right now
        circles.push(circle_new);
        i += 1;
    }
    circles
}

fn generate_circle() -> Circle {
    let mut rng = thread_rng();
    // First x and y position 
    let pos_x: f32 = rng.gen_range(RADIUS, SCREEN_WIDTH as f32 - RADIUS);
    let pos_y: f32 = rng.gen_range(RADIUS, SCREEN_HEIGHT as f32 - RADIUS);
    // Then velocity in x and y
    let dir_x: f32 = rng.gen_range(-3.0, 3.0);
    let dir_y: f32 = rng.gen_range(-3.0, 3.0);
    let circle_new = Circle::new(pos_x as f32, pos_y as f32, RADIUS, dir_x as f32, dir_y as f32);
    circle_new
}

// True if there is an intersection
fn check_intersections(circle_new: &Circle, circles: &Vec<Circle>) -> bool {
    for circle in circles {
            if circle.check_intersect(&circle_new) {
                return true;
            }
        }
    false
}
*/