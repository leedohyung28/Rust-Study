fn main() {
    step1_1();
}

fn step1_1() {
    // println_test();
    // todo_test();
    // print_test();
    // brackets_test();
    
    // 모든 Rust 프로그램에는 이름이 main인 함수가 한 개만 있어야 한다.
}
fn println_test() {
    println!("Hello, world!");
    // Rust는 println를 통해 결과 도출
    // 세미콜론(;) 사용하여 문장 종료
}
fn todo_test() {
    todo!("Display the message by using the prinln!() macro");
    // Rust 프로그램에서 완성되지 않은 코드를 식별하는데에 사용
    // 컴파일러가 완료된 기능을 찾을 것으로 예상되는 패닉 메시지 반환
}
fn print_test() {
    print!("This is first line.");
    println!("This is second line.");
    print!("This is third line.");
    // print!와 println!의 차이
    // print! -> 출력 후 줄바꿈 X
    // println! -> 출력 후 줄바꿈 O
}
fn brackets_test() {
    println!("The first letter of the English alphabet is {} and the last is {}", "A", "Z");
    // Rust에서는 {}를 통해 그 다음 인수 값을 호출
}
