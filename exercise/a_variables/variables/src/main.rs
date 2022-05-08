const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", missiles.0, missiles.1);
    missiles.0 = missiles.0 - missiles.1;
    println!("{} missiles left", missiles.0);
}
