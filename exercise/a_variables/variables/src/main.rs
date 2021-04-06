
const STARTING_MISSILES :i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    let jet = 1;

    println!("Firing {} of my {} missiles...!", ready, missiles);

    println!("{} missles left", missiles - ready);
}
