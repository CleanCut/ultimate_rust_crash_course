// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    // 2. 계산된 영역이 올바르지 않습니다! 아래의 area_of() 함수를 수정한 다음 코드를 다시 실행하고 올바르게 작동하는지 확인하세요 (영역은 28이어야 합니다).

    // 3. 아래의 줄 주석을 해제하세요. `volume` 함수가 아직 작동하지 않습니다.
    //    `volume` 함수를 생성하세요! 다음을 따라야 합니다:
    //    - i32 타입의 세 개의 인수를 가져야 합니다.
    //    - 세 인수를 곱해야 합니다.
    //    - 결과를 반환해야 합니다 (프로그램을 실행할 때 결과는 280이어야 합니다).
    //
    // 곤란할 경우, 이것이 `area_of`가 하는 것과 *매우* 유사하다는 것을 기억하세요.
    //
    //println!("체적은 {}", volume(width, height, depth));

}

fn area_of(x: i32, y: i32) -> i32 {
    x*y
}