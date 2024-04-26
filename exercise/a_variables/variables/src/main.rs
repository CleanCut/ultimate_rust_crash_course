const STARTING_MISSILE: i32 = 8;
const READY_AMOUNT: i32 = 2; // const는 type을 명시적으로 적어줘야 한다.

fn main() {

    let (mut missiles, ready): (i32, i32)  = (STARTING_MISSILE, READY_AMOUNT);

    println!("내 {}개의 {} 미사일을 발사 중...", ready, missiles);

    missiles -= ready;
    println!("{}개의 미사일이 남았습니다.", missiles);
}
