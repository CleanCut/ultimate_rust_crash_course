// Someone is shooting arrows at a target.  We need to classify the shots.
//
// 1. Create an enum called "Shot" with variants:
// - Bullseye
// - Hit, containing the distance from the center (an f64)
// - Miss


impl Shot {
    // Here is a method for the Shot enum you just defined.
    fn points(self) -> i32 {
        // 2. Implement this method to convert a Shot into points
        // - 5 points for Bullseye
        // - 2 points for Hit(x) where x < 3.0
        // - 1 point for Hit(x) where x >= 3.0
        // - 0 points for Miss
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on
    // the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 3. For each Coord in arrow_coords:
    //
    //   A. Call coord.print_description()
    //   B. Create the correct variant of Shot depending on the value of
    //      coord.distance_from_center()
    //      - Less than 1.0 -- Bullseye
    //      - Between 1.0 and 5.0 -- Hit(value)
    //      - Greater than 5.0 -- Miss


    let mut total = 0;
    // 4. Finally, loop through each shot in shots and add its points to total

    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}