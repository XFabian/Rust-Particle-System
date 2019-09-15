// Used for spatial Hashing
// Need to implement iteration on the map
// hash is just a map which each contains a list
// the hash for the map is a rectanglöe/border in spatial coordinates. This means we split the screen in sepeerate foels
use std::collections;
use crate::circle::{Circle, Vertex};
pub struct SpatialHash<> {
    width: f32, // Here we have a problem with coords top right is (-1,1) and bottom left is (1, -1) and our width si 2
    height: f32,
    cell_size: f32,
    pub cell_map: collections::HashMap<(i32, i32), Vec<Circle>>,
}
// Maybe i can do calculations fully symmetric 
impl SpatialHash{

    pub fn new() -> SpatialHash {
        SpatialHash {width: 1.0,
                    height: 1.0,
                    cell_size:0.5, // this should give us a split with 16 since we mirror into negative
                    cell_map: collections::HashMap::new()} 
    }

    // init all keys in the Hash map. I can precompute all keys (x,y) starting at 1 and are until (width / cell_size) +1. and this also negeatve -1 to 
    pub fn init_hashmap(&mut self) {
        let mut cell_map: collections::HashMap<(i32, i32), Vec<Circle>> = collections::HashMap::new();
        let cols = (self.width / self.cell_size) as i32;
        let rows = (self.height / self.cell_size) as i32;
        for i in 1..(cols+1) {
            for j in 1..(rows+1) {
                cell_map.insert((i, j), Vec::new());
                cell_map.insert((i, -j), Vec::new());

                cell_map.insert((-i, j), Vec::new());
                cell_map.insert((-i, -j), Vec::new());
            }
            
        }
        self.cell_map = cell_map;
    }
    // return vertices ore vertex buffer to draw the borders of our spatial hash. Used for debugging Thses vertices are actually fix and cannot be changed so can make this constant
    pub fn draw_borders(&self) -> Vec<Vertex>{
        // create vertices for columns
        let mut shape: Vec<Vertex> = Vec::new();
        let cols = (self.width / self.cell_size) as i32; // careful we need to double these because of negative range
        let rows = (self.height / self.cell_size) as i32;
        let mut x_vert = -1.0;
        for x in 0..(2*cols) { // we jsut want to draw some lines
             // start point of the lines y coordinate is always 1 or -1
            let start = Vertex { position: [x_vert, 1.0]};
            let end = Vertex { position: [x_vert, -1.0]};
            shape.push(start);
            shape.push(end);
            x_vert += self.cell_size;
        }
        // create vertices for rows
        let mut y_vert = -1.0;
        for y in 0..(2*rows) {

            let start = Vertex { position: [1.0, y_vert]};
            let end = Vertex { position: [-1.0, y_vert]};
            shape.push(start);
            shape.push(end);
            y_vert += self.cell_size; // start point of the row lines x coordinate is always 1 or -1
        }
        // create bounding box very ugly
        let x = 1.0;
        let y = 1.0;
        let start = Vertex { position: [x, y]};
        let end = Vertex { position: [-x, y]};
        shape.push(start);
        shape.push(end);

        let start = Vertex { position: [x, -y]};
        let end = Vertex { position: [-x, -y]};
        shape.push(start);
        shape.push(end);

        let start = Vertex { position: [x, -y]};
        let end = Vertex { position: [x, y]};
        shape.push(start);
        shape.push(end);

        let start = Vertex { position: [x, -y]};
        let end = Vertex { position: [x, y]};
        shape.push(start);
        shape.push(end);
            
        
        shape
    }

    fn hash_id(&self, circle: &Circle) -> (i32, i32){
        // if circle coordinate is negatove we know it is in the other direction and need to multiply by -1 
        // (1,1) will be the first bucket no 0 buckets because we need the sign
        // first we cale everything up by 100 so we can calculate better with int buckets
        let cell_size_scaled = (self.cell_size * 100.0) as i32;
        let mut pos_x_scaled = (circle.position.x * 100.0) as i32;
        let mut pos_y_scaled = (circle.position.y * 100.0) as i32;
        
    	let mut sign_x = pos_x_scaled.signum(); // returns the sign of x
        let mut sign_y = pos_y_scaled.signum();
        // Probably more rustire solution here possible
        while pos_x_scaled.abs() >= 100 {
            pos_x_scaled += -1 * sign_x * 1;
        }
        while pos_y_scaled.abs() >= 100 {
            pos_y_scaled += -1 * sign_y * 1;
        }
        if pos_x_scaled == 0 {
            sign_x = 1;
        }

        if pos_y_scaled == 0 {
            sign_y = 1;
        }
        let cell_position_x: i32 = ((pos_x_scaled.abs() + cell_size_scaled) / cell_size_scaled) * sign_x; // times sign because we have to go in both directions since coordinate system is in middle
        let cell_position_y: i32 = ((pos_y_scaled.abs() + cell_size_scaled) / cell_size_scaled) * sign_y; // here math floor is needed
        /*println!("Hashed entry {} {}", cell_position_x, cell_position_y);
        if cell_position_x == 0 {
            println!("x_abs {} cell_size{} sign{} add{}", pos_x_scaled.abs(), cell_size_scaled, sign_x, pos_x_scaled.abs() + cell_size_scaled);
        }
        if cell_position_y == 0 {
            println!("y_abs {} cell_size{} sign{} add{}", pos_y_scaled.abs(), cell_size_scaled, sign_y, pos_y_scaled.abs() + cell_size_scaled);
        }
        if cell_position_y == 3 {
            println!("y_abs {} cell_size{} sign{} add{}", pos_y_scaled.abs(), cell_size_scaled, sign_y, pos_y_scaled.abs() + cell_size_scaled);
            println!("Not rounded {}", circle.position.y);
        }
        if cell_position_x == 3 {
            println!("x_abs {} cell_size{} sign{} add{}", pos_x_scaled.abs(), cell_size_scaled, sign_x, pos_x_scaled.abs() + cell_size_scaled);
            println!("Not rounded {}", circle.position.x);
        }
        if cell_position_x == -3 {
            println!("x_abs {} cell_size{} sign{} add{}", pos_x_scaled.abs(), cell_size_scaled, sign_x, pos_x_scaled.abs() + cell_size_scaled);
            println!("Not rounded {}", circle.position.x);
        }
        if cell_position_y == -3 {
            println!("y_abs {} cell_size{} sign{} add{}", pos_y_scaled.abs(), cell_size_scaled, sign_y, pos_y_scaled.abs() + cell_size_scaled);
            println!("Not rounded {}", circle.position.y);
        }*/
        (cell_position_x, cell_position_y)
    }

    pub fn add(&mut self, circle: Circle) {
        let hash = self.hash_id(&circle);
        //println!("{:?}", hash);
        if let Some(vec) = self.cell_map.get_mut(&hash){ // since all keys are here this shoudl always work
            //println!("Adding to vector");
            vec.push(circle);
        }
        else {
            println!("Insertion failed!");
            panic!();
        }
        //self.cell_map.insert(hash, Vec::new());
    }

    pub fn reset(&mut self) {
        for (bin, list) in &mut self.cell_map {
            list.clear();
        }
    }
    // Since a circle can be at the border of a region we need to add it to both regions  check every direction and if it overlaps we add it to this region
    // Or just insert once and also check sorrounding bins
    pub fn iterate_map(&self) {
        for (bin, list) in &self.cell_map {
            println!("cell: {:?} and {:?}", bin, list);
        }
    }
    // TODO add hash function
    // add and remove objects
    // get iterator over the hash map
}