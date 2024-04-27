// 연습을 방해하지 않도록 일부 경고를 무시합니다.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // 커맨드 라인 인수를 String 벡터로 수집합니다.
    // 예를 들어:
    //
    //     cargo run apple banana
    //
    // ...는 다음과 같습니다:
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // 이는 `args` 벡터를 소비하여 각 String을 반복합니다.
    for arg in args {
        // 1a. 당신의 임무: 커맨드 라인 인수를 처리하세요!
        //
        // - 만약 arg가 "sum"이라면, sum() 함수를 호출하세요
        // - 만약 arg가 "double"이라면, double() 함수를 호출하세요
        // - 그 외의 경우, count() 함수를 호출하되, "arg"를 전달하세요.
        // if arg == "sum" {
        //     sum()
        // } else if arg == "double" {
        //     double();
        // }
        // } else {
        //     count(arg);
        // }

        match arg.as_str() {
            "sum" => sum(),
            "double" => double(),
            _ => count(&arg),
        }


        // 1b. 이제 "sum", "double" 및 "bananas"를 프로그램에 전달하여 인수를 추가해 보세요
        // "cargo run" 다음에 예를 들면 "cargo run sum"
    }
}

fn sum() {
    let mut sum = 0;
    for i in 7..24 {
        sum += i;
    }
    // 2. 7부터 23까지 *포함하여* 정수를 반복하여 모두 더합니다 (sum 변수를 증가시킵니다). 
    // 팁: 255가 나와야 합니다
    // "cargo run sum"으로 실행하세요


    println!("합계는 {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. `x`의 값을 2배씩 곱하여 `x`가 500보다 큰 경우까지 몇 번 반복할 수 있는지 세는 "while 루프"를 사용하세요.
    // 각 루프를 통과할 때마다 `count`를 증가시킵니다. "cargo run double"로 실행하세요
    // 팁: 답은 9번입니다.

    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("x가 500보다 큰 경우까지 x를 2배씩 할 수 있는 횟수는 {}", count);
}

fn count(arg: &String) {
    // 도전 과제: `arg`를 8번 출력하고 그 후에 루프를 종료하기 위해 무조건적인 루프 (`loop`)를 사용하세요.
    // 어떻게든 루프를 세어야 할 것입니다. "cargo run bananas"로 실행하세요
    //
    // print!("{} ", arg); // 이 라인을 8번 실행한 후에 루프를 종료하세요. `print!`는 새 줄을 추가하지 않습니다.

    let mut count = 0;
    loop {
        count += 1;
        print!("{} ", arg);
        if count == 8 {
            break;
        }
    }

    println!(); // 마지막에 깔끔하게 개행만 출력합니다.
}
