// 일부 경고를 무시하도록 설정하여 연습에 방해가 되지 않도록 합니다.
#![allow(dead_code, unused_variables)]

// use ding_machine::print_difference;
// use crate::print_difference;

// use my_lib::{ding, on_off, print_array, print_difference};
use ding_machine::{ding, on_off, print_array, print_difference};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

  // 1. `coords`의 일부분을 `print_difference` 함수에 전달합니다. 
  // 이 함수는 `cargo run`을 실행할 때 coords에 있는 두 수의 차이를 보여줘야 합니다. 튜플 인덱싱을 사용하세요.
  //
  // `print_difference` 함수는 `main` 함수 아래에 정의되어 있습니다. 함수가 어떻게 정의되어 있는지 살펴보면 도움이 될 수 있습니다.
  //
  // print_difference( ... );  // 주석 해제하고 이 줄을 완성하세요
    print_difference(coords.0, coords.1); // 주석 해제하고 이 줄을 완성하세요

  // 2. `print_array` 함수를 사용하여 coords를 출력하고 싶지만 coords는 배열이 아닙니다!
  // [f32; 2] 타입의 배열을 만들고 coords의 정보를 포함하도록 초기화하십시오. `print_array` 라인의 주석을 해제하고 코드를 실행하십시오.
  //
  // let coords_arr...        // 여기에 `coord`의 일부분으로부터 배열 리터럴을 만드세요
  // print_array(coords_arr);    // 여기에 넘겨주세요 (이 줄은 변경할 필요 없음)
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];

  // 3. `ding` 함수를 만족스럽게 하려면 `series` 배열에서 값 13을 넘겨주세요.
  // 배열 인덱싱을 사용하세요. 올바르게 수행되면 `cargo run`은 "Ding, you found 13!"라는 추가 출력을 생성합니다.
  //
  // ding(...);
    let a = &series[series.len()-1..];
    ding(a[0]);
    let b = series[series.len()-1];
    ding(b);
    match series.last() {
        Some(last_number) => ding(*last_number),
        None => print!("Error!")
    }

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");

  // 4. `on_off` 함수에 변수 `mess`에서 값 `true`를 전달하세요. 올바르게 수행되면 
  // `cargo run`은 "Lights are on!"이라는 추가 출력을 생성합니다. 시작해 드리겠습니다:
  //
  // on_off(mess.2 ...);
    on_off(mess.2[1].0);

  // 5.  너무 복잡한 코드 - 함수들이 섞여있네요! 코드를 정리.
  //
  // - 라이브러리 파일(src/lib.rs) 만들기
  // - 모든 함수(main 제외)를 라이브러리로 이동
  // - 모든 함수를 `pub` 키워드를 사용하여 공개 함수로 만들기
  // - `use` 문을 사용하여 모든 함수를 스코프로 가져오기. 라이브러리의 이름은 Cargo.toml에 정의되어 있습니다. `use`하기 위해서는 이 이름을 알아야 합니다.
  //
  // `cargo run`은 동일한 출력을 생성해야 하며, 이제 코드는 더욱 조직적입니다. 

  // 챌린지: 아래 줄의 주석을 해제하고 코드를 실행한 다음 출력을 살펴보세요. 
  // 그런 다음 해당 함수 내의 주석 지침에 따라 print_distance() 함수를 리팩터링하세요.

  // print_distance(coords);
}
