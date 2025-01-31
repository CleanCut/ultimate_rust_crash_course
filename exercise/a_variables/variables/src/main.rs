const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let mut missiles:i32 = STARTING_MISSILES;
    // let ready:i32 = READY_AMOUNT;
    // let (mut missiles, ready):(i32,i32) = (STARTING_MISSILES, READY_AMOUNT); // This is cool!
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let _dummy: i32;
    println!("Firing {} of my {} missiles...", ready, missiles);
    //missiles -= ready;
    println!("{} missiles left...", missiles - ready);
}
