#[macro_use]
extern crate glium;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::env;

use glium::{glutin, Surface};
mod vec2;
mod circle;
mod spatialHashing;
mod shaders;
use shaders::{Attr_line, AttrCircle};
use circle::{Circle, Vertex, RADIUS, SCREEN_HEIGHT, SCREEN_WIDTH, LINE_THRESHOLD};
use spatialHashing::{SpatialHash};

use std::collections::hash_map::Entry::{Vacant, Occupied};

// Lags > 40
// 1. Profile the Code and see where the overhead is => atleast works when I optimize with cargo run --release
// 2. Use textures on the lines and the circles and make it look prettier
// 3. Paeticle effects when two circles collide
// 4. Check out repo for glium_shapes and maybe use these to create the shapes with the indices and check if it is alos so slow
// 5. Add in Key commands again => Done
// 6. Add Quad Trees. This means a broad phase when there are many circles on screen. Also R trees spatial hashing 
// 7. Fix timing in the loop and create a delta time variable => maybe done atleast ii got a time
// Alpha wert random 
// Linien dicker
// weißer hintergrun schwarze kugeln
// simulator struct? z.b mit linine oder ohne hintergrund farbe etc 
// Show collisions before and after doing spatial hashing using glium text library
// When I hover mouse specific bounding box should light up in another color 
fn main() {

    let args: Vec<String> = env::args().collect();
    let circle_number: i16 = args[1].parse().unwrap();
    let draw_params_circle = glium::DrawParameters {
                blend: glium::Blend {
                    color: glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                    },
                    alpha: glium::BlendingFunction::Addition {
                                source: glium::LinearBlendingFactor::SourceAlpha,
                                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
                            },
                    constant_value: (0.0, 0.0, 0.0, 0.0), // I dunno what this is used for
                },
                .. Default::default()
            };
    let mut events_loop = glium::glutin::EventsLoop::new();

    let wb = glutin::WindowBuilder::new()
        .with_title("Particles");
        //.with_dimensions(glutin::dpi::LogicalSize::new(1800.0, 1024.0));;
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut target = display.draw(); // Create frame to draw on
    target.clear_color(1.0, 1.0, 1.0, 1.0); // RGBA between 0 and 1
    target.finish().unwrap();
    let mut test = SpatialHash::new();
    let mut cir2 = Circle::new(0.1, 0.1,RADIUS, 0.01, 0.01);
    let mut cir3 = Circle::new(0.1,0.6,RADIUS, 0.01, 0.01);
    let mut cir4 = Circle::new(-0.9,0.9,RADIUS, 0.01, 0.01);
    test.init_hashmap();
    //test.draw_borders();
    test.iterate_map();
    test.reset();
    test.iterate_map();
    //panic!();
    let vert_buf_border: glium::VertexBuffer<circle::Vertex> = glium::VertexBuffer::new(&display, &test.draw_borders()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
    let indices_lines = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let program = glium::Program::from_source(&display, shaders::vertex_shader_src, shaders::fragment_shader_src, None).unwrap(); // defines a program using our vertices and pixel shader
    let program_lines = glium::Program::from_source(&display, shaders::vertex_line_shader_src, shaders::fragment_line_shader_src, None).unwrap();
    let program_border =  glium::Program::from_source(&display, shaders::vertex_shader_src, shaders::fragment_line_shader_src, None).unwrap();
    let mut closed = false;
    let mut circles = create_random_circles(circle_number);
    let mut comparisons = 0;
    let linear_interpolate = |x: f32| {
        let y = (1.0 * (LINE_THRESHOLD - x) + 0.0 * (x - 0.0)) / (LINE_THRESHOLD - 0.0);
        y
    };
    
    let next_frame_time = std::time::Instant::now();
    let mut spatial_map = SpatialHash::new();
    spatial_map.init_hashmap();
    

    let mut line_alpha_vec: Vec<Attr_line> = Vec::new();
    let vert_buf_lines: glium::VertexBuffer<circle::Vertex> = glium::VertexBuffer::new(&display, &[
                                        Vertex {position: [0.0, 0.0]},
                                        Vertex {position: [0.0, 1.0]},
                                    ]).unwrap();
    let mut circle_attrs: Vec<AttrCircle> = Vec::new();
    let mut cir_cent = Circle::new(0.0, 0.0, RADIUS, 0.01, 0.01); // we can ignore velocity 
    let cir_cent_shape = cir_cent.create_circle();
    let vert_buf_circle: glium::VertexBuffer<circle::Vertex> = glium::VertexBuffer::new(&display, &cir_cent_shape).unwrap();
    while !closed {
        // TODO if there is only one object in bin and another one comes the comparison does not work. There has to be 3 things in the bin
        //println!("Delta time:{:?}", next_frame_time.elapsed().subsec_nanos() as f64 * 1e-9);
        let delta_time = next_frame_time.elapsed().subsec_nanos() as f64 * 1e-9; // delta time in seconds
        comparisons = 0;
        //::std::thread::sleep(std::time::Duration::from_nanos(16_666_667)); //  //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); exactly this division
        let mut target = display.draw(); // Create frame to draw on
        target.clear_color(1.0, 1.0, 1.0, 0.0); // RGBA between 0 and 1
        //target.draw(&vert_buf_border, &indices_lines, &program, &glium::uniforms::EmptyUniforms,
        //    &Default::default()).unwrap();
        /*
        for i in 0..circles.len() {
            let circle = circles.remove(0);
            spatial_map.add(circle);
        }
        for bin in &mut spatial_map.cell_map.keys() {
            match spatial_map.cell_map.entry(*bin) { // entry returns an option that we can use to do in place operations on the list
                Occupied(list_) => {
                    let list = list_.get(); // map should just give me the list I could just return it 
                    if list.len() == 0 {
                        continue;
                    }
                    if list.len() == 1 {
                        circles.append(&mut list);
                        continue;
                    }
                    for i in 0..list.len() {
                        let mut circle = list.remove(0); // drain the vector
                        //println!("{}", list.len());
                        // Maybe just create two ranges and do all the checks in there
                        //let mut rest_list: Vec<_> = list.drain(0..).collect(); // I think this drain makes probelms when therea re only two elements in the list one is removed and drain returns nothing
                        for mut circle_comp in  list {
                            comparisons += 1;
                            if circle.check_intersect(&circle_comp) { // if they intersect both circles move ito the different direction
                                //println!("Collision");
                                Circle::resolveCollision(&mut circle, &mut circle_comp);
                            }
                            let (draw_line, distance) = circle.check_draw_line(&circle_comp);
                            if draw_line {
                                let predi_pos = circle.predicted_move(); // use predicted position so line is centred in circle
                                let predi_pos2 = circle_comp.predicted_move();
                                vert_buf_lines.write(&[
                                                        Vertex {position: [predi_pos.x, predi_pos.y]},
                                                        Vertex {position: [predi_pos2.x, predi_pos2.y]},
                                                    ]);
                                let alpha = linear_interpolate(distance);
                                target.draw(&vert_buf_lines, &indices_lines, &program_lines, &uniform! {alpha: alpha}, // uniform is global variable which value set when we draw call and inserted in shader functions
                                    &draw_params_circle).unwrap();
                            }
                        }
                        list.push(circle); // add again at the end
                    }
                    // add all elemets to the circle again
                    circles.append(&mut list);
                    
                    
                },
                
                Vacant(_) => {

                }
            }
        }*/
            // Some values are mapped to a 0 key 2
            // Neighbor check has to include neighbouring bins aswell 
            
        
        // Then we check if circles intersect and if yes we invert the direction and move
        
        for i in 0..circles.len() {
            let mut circle = circles.remove(0); // start at beginning and get each element. Since remove pushes all to the left we always take 0s element

            // check interseciton with rest of the circles and draw lines
            for circle_comp in &mut circles {
                comparisons += 1;
                if circle.check_intersect(&circle_comp) { // if they intersect both circles move ito the different direction
                    Circle::resolveCollision(&mut circle, circle_comp);
                }
                let (draw_line, distance) = circle.check_draw_line(&circle_comp);
                if draw_line {
                    let predi_pos = circle.predicted_move(); // use predicted position so line is centred in circle
                    let predi_pos2 = circle_comp.predicted_move();
                    let alpha = linear_interpolate(distance);
                    line_alpha_vec.push(Attr_line {alpha: alpha,
                                                    position_s: [predi_pos.x, predi_pos.y],
                                                    position_e: [predi_pos2.x, predi_pos2.y]});                       
                }
            } 
            circles.push(circle); // push circle again into vector at the end

        }
        // Instancing works if I have one line and then apply stuff on these lines
        let mut per_instance_line_attr = glium::vertex::VertexBuffer::new(&display, &line_alpha_vec).unwrap();
        target.draw( (&vert_buf_lines, per_instance_line_attr.per_instance().unwrap()),  
        &indices_lines, &program_lines, &glium::uniforms::EmptyUniforms, // uniform is global variable which value set when we draw call and inserted in shader functions
                       &draw_params_circle).unwrap();
        line_alpha_vec.clear();

        for circle in &mut circles { // check intersection with all other circles
            circle.check_bounds(); // only move the ones that are flagged when they intersected because they need to move again
            circle.move_circle();
            circle.compute_alpha(); // apply step size and bounds cheking to alpha value
            circle_attrs.push(AttrCircle {position_c: [circle.position.x, circle.position.y],
                                           alpha: circle.alpha,
                                                });   
        }
        let mut per_instance_circle_attr = glium::vertex::VertexBuffer::new(&display, &circle_attrs).unwrap(); // fixed size so better and just change when adding circle
        target.draw( (&vert_buf_circle, per_instance_circle_attr.per_instance().unwrap()),  
                    &indices, &program, &glium::uniforms::EmptyUniforms,
                       &draw_params_circle).unwrap();
        circle_attrs.clear();
        //println!("Number of comparisons: {}", comparisons);
        target.finish().unwrap(); // destroys frame object and copies imafe to the window

        events_loop.poll_events(|ev| {
            match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        glutin::WindowEvent::KeyboardInput {input:
                                                                glutin::KeyboardInput {
                                                                    virtual_keycode: Some(virtual_code),
                                                                    state,
                                                                    ..
                                                                }, ..}
                                => match (virtual_code, state) {
                                (glutin::VirtualKeyCode::Escape, _) => closed = true,
                                (glutin::VirtualKeyCode::N, glutin::ElementState::Pressed) => {println!("Add random circle to screen");
                                                                                                let mut circle_new = generate_circle();
                                                                                                while check_intersections(&circle_new, &circles) {
                                                                                                    println!("In while loop");
                                                                                                    circle_new = generate_circle();
                                                                                                }
                                                                                                circles.push(circle_new); }// Has no check right now if there is a circle right now},
                                (glutin::VirtualKeyCode::R, glutin::ElementState::Pressed) => {if circles.len() == 0 {}
                                                                                                else {
                                                                                                println!("Remove random circle");
                                                                                                let mut rng = thread_rng();
                                                                                                let idx_remove = rng.gen_range(0, circles.len());
                                                                                                circles.remove(idx_remove);}
                                                                                                },
                            _ => (),
                            },
                    _ => {},
                    }
                
                _ => (),
                }
        });

        
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // locked to 60 frames per second
    }

}

// Fuck this here  just need the triangle fan method 
fn draw_circle(cx :f32, cy: f32, r: f32, num_segments: i32, num_circle: u32) -> (Vec<Vertex>, Vec<u32>) {
    // num circle used to specify which circle we are at for inidces
    let mut indices: Vec<u32> = Vec::new();
    let mut shape: Vec<Vertex> = Vec::new();
    for i in 0..num_segments {
        let theta = 2.0 * 3.1415926 * (i as f32) / (num_segments as f32); // current angle

        let x = r * theta.cos();
        let y = r * theta.sin();
        let v1 = Vertex { position: [x + cx, y+ cy]};
        shape.push(v1);
    }
    println!("{}", shape.len());
    let number_vertices :u32 = 12; // number of vertices in the shape but the shape is [0,11] 
    let offset: u32 = num_circle * number_vertices; // used for each other circle 
    // create indices 
    for j in 0..(number_vertices - 2) { // - 2 since the fan needs that many triangles and -1 because we start at 0
        indices.push(offset); // first coordinate is always 0
        indices.push(j + offset + 1);
        indices.push(j + offset + 2);
        println!("{} {} {}", offset, j + offset + 1, j + offset + 2)
    }
    (shape, indices)
}

// Create valid circles at random positon with random velocity. The velocity is between [-3, 3] in both directions. number is the number of circles to create
fn create_random_circles(number: i16) -> Vec<Circle> {
    let mut circles: Vec<Circle> = Vec::new();
    
    let mut i = 0;
    while i < number {
        
        let circle_new = generate_circle();
        let intersects = check_intersections(&circle_new, &circles);
        
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
    let pos_x: f32 = rng.gen_range(-1.0 + RADIUS, SCREEN_WIDTH as f32 - RADIUS);
    let pos_y: f32 = rng.gen_range(-1.0 + RADIUS, SCREEN_HEIGHT as f32 - RADIUS);
    // Then velocity in x and y
    let dir_x: f32 = rng.gen_range(-0.005, 0.005);
    let dir_y: f32 = rng.gen_range(-0.005, 0.005);
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