fn main() {
    // step1_1();
    // step1_2();
    // step1_3();
    step1_4();
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

fn step1_2() {
    // let_test();
    // mut_test();
    // variable_shadowing_test();
}
fn let_test() {
    let a_number;
    a_number = 10;
    // let은 변수를 declare하고 bind할 수 있다.

    let a_word = "Ten";
    // let은 변수를 declare하는 동시에 bind할 수도 있다.

    println!("The number is {}", a_number);
    println!("The word is {}", a_word);
}
fn mut_test() {
    let mut a_number = 10;
    // mut은 변수를 바꿀 수 있다.
    println!("Previous num : {}", a_number);
    a_number = 15;
    println!("Now num : {}", a_number);
}
fn variable_shadowing_test() {
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("Result : {}", shadow_num);
    // 기존 변수의 이름을 활용하는 새 변수를 선언 가능하다.
    // 이를 Rust에서는 "Shadowing"이라고 한다.
}

fn step1_3() {
    // num_type_test();
    // float_type_test();
    // bool_test();
    // char_test();

    // Rust에서 char는 알파벳, 숫자, 이미지 등을 비롯한 모든 단일 문자
}
fn num_type_test() {
    let number: u32 = 14;
    println!("The number is {}.", number);

    // 길이   : 부호O,  부호X
    // 8비트  :    i8,    u8
    // 16비트 :   i16,   u16
    // 32비트 :   i32,   u32
    // 64비트 :   i64,   u64
    // 128비트:  i128,  u128
    // 종속   : isize, usize
}
fn float_type_test() {
    // 10지수 : f32, f64 존재 (기본 부동 소수점 형식 : f64)
    // f64는 f32와 속도는 비슷 but 더욱 정밀

    let number_64 = 4.0;    // compiler가 자동적으로 f64로 인식
    let number_32: f32 = 5.0;    // f32가 되도록 명시적 선언

    println!("1+2 = {} and 8-5 = {} and 15*3 = {}", 1u32 + 2, 8i32 - 5 , 15 * 3);
    println!("9/2 = {} but 9.0/2.0 = {}", 9u32 / 2, 9.0/2.0);
    // 정수와 FLoating point의 나눗셈 차이
}
fn bool_test() {
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);
    // false라고 출력

    let is_smaller = 1 < 4;
    println!("Is 1 < 4? {}", is_smaller);
}
fn char_test() {
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';
    // 모든 텍스트 형식은 유효한 UTF-8 표현
    // char는 너비가 32비트가 되도록 채워지는 21비트 정수.
    // char는 일반 코드 포인트 값을 직접 포함

    let string_1 = "miley";
    let string_2 = "ace";
    // 'str'형식은 문자열 조각으로 '&str'가 있는 형식 앞의 참조 스타일 구문 사용
    // String : 프로그램이 실행될 때 변경될 수 있는 텍스트 데이터
    // &str : 프로그램이 실행될 때 변하지 않는 텍스트 데이터에 대한 변경 불가능한 보기

    println!("{} is a {}{}{}{}.", smiley_face, uppercase_s, string_1, lowercase_f, string_2);
}

fn step1_4() {
    // tuple_test();
    // struct_test();

    // 튜플은 다양한 형식의 값 컬렉션이며, 길이는 요소 수에 따라 고정된다.
    // 구조체의 각 필드에는 이름과 데이터 형식이 있지만, 튜플 구조체의 필드에는 이름이 없다.
}
fn tuple_test() {
    // 튜플 : 하나의 복합 값으로 수집되는 다양한 형식을 그룹화
    // (<value>, <value>, ...)
    // 괄호로 묶음, 쉼표로 구분
    // 요소의 수와 동일한 고정 길이
    // 선언 이후 크기 확대X, 축소X, 요소 추가X, 요소 제거X
    
    let tuple_1 = ('E', 5i32, true);
    // 요소     값      데이터 형식
    //   0      E            char
    //   1      5             i32
    //   2   true            bool

    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_1.0, tuple_1.1, tuple_1.2);
    // tuple은 요소에 액세스할 때
    // <tuple>.<index>를 사용한다.
}
fn struct_test() {
    // 구조체 : 다른 형식으로 구성된 형식
    // 이름으로 구조체 정의 -> 각 필드에 대한 데이터 형식 지정 -> 구조체의 인스턴스 생성 -> 인스턴스 선언 시 필드의 특정 값 제공

    // 구조체의 종류
    // 1. 클래식 C 구조체 : 이름과 데이터 형식, 정의 후 <struct>.<field>를 사용하여 액세스
    // 2. 튜플 구조체 : 클래식 구조체와 유사 but 필드에 이름 X, <tuple>.<index>를 사용, 인덱스 값 0부터 시작
    // 3. 단위 구조체 : 가장 일반적인 사용

    // 1. 클래식 구조체
    struct Student { name: String, level: u8, remote: bool}

    // 2. 튜플 구조체
    struct Grades(char, char, char, char, f32);

    // 3. 단위 구조체
    struct Unit;

    let user_1 = Student { name: String::from("Dohyung Lee"), remote: true, level: 2};
    let user_2 = Student { name: "James Maddison".to_string(), level: 7, remote: false};
    // 문자열 리터럴을 문자열 형식으로 반환
    // .to_string()또는 String::from(&str)을 통해 반환 가능
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);
    println!("{}, level {}. Remote: {}, Grades: {}, {}, {}, {}. Average: {}",
user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
println!("{}, level {}. Remote: {}, Grades: {}, {}, {}, {}. Average: {}",
user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}

