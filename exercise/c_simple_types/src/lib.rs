
pub fn print_difference(x: f32, y: f32) {
  println!("{}과 {}의 차이는 {}", x, y, (x - y).abs());
}

pub fn print_array(a: [f32; 2]) {
  println!("좌표는 ({}, {})입니다.", a[0], a[1]);
}

pub fn ding(x: i32) {
  if x == 13 {
    println!("Ding, you found 13!");
  }
}

pub fn on_off(val: bool) {
  if val {
    println!("Lights are on!");
  }
}

pub fn print_distance(z: (f32, f32)) {
  // z.0과 z.1을 사용하는 것은 x와 y를 사용
}