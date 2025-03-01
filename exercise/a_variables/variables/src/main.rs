const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
    missiles -= ready;
    println!("{} missiles left", missiles);
}
