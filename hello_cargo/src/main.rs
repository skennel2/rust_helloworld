mod modtest;

fn main() {
    print_hello_world();
    primitive_type();
    mutabillity();
    string_type();
    copy_value();
    move_memory();
    declare_first();
    ownership();
    borrow();
    mut_reference();
    reference_limit();
    struct_test();

    for_loop();
    match_syntax();
    println!("{}", modtest::get_hello_world());
}

fn print_hello_world() {
    let value = "Hello World";

    println!("{}", value);
}

/**
 * signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
 * unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
 * floating point: f32, f64
 * char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
 * bool either true or false
 * and the unit type (), whose only possible value is an empty tuple: ()
 * 
 */
fn primitive_type() {
    let boolean: bool = true;
    let a_float:f64 = 3.14;

    // suffix 사용
    let an_integer = 5i32;
    
    // 타입추론 기본값
    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12; // 아래 라인으로 인해 i64로 추론된다.
    inferred_type = 4294967296i64;

    let a_char: char = 'a';
}

fn string_type() {
    //??? 스트링리터럴과 스트링타입이 별도로 분리되어 있는듯하다.
    let literal_string = "hello string literal";
    let mut string_type = String::from("Hello String");
    //??? 새로운 스트링을 리턴하는것이 아닌 값 자체를 변경시키는듯하다.
    string_type.push_str("!!!");
    println!("{}", literal_string);
    println!("{}", string_type);
}

fn copy_value() {
    let origin = 5;
    // 일반적인 언어의 개념과 동일하게 값 복사가 이루어진다.
    let copy = origin;

    println!("{}", origin);
    println!("{}", copy);
}

fn move_memory() {
    {
        let origin = String::from("will be moved");

        let target = origin;

        println!("{}", target);

        // 아래 주석을 풀면 컴파일 에러
        // 스택에서의 메모리 주소 복사가 아닌 이동의 개념이다.
        // 한번 이동된 것은 컴파일러가 유효하지 않다고 판단한다.
        // println!("{}", origin);
    }
}

fn declare_first() {
    let a_binding;
    {
        let x = 2;

        a_binding = x * x;
    }

    println!("{}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn ownership() {
    let s = String::from("ownership will be moved to some function");

    takes_ownership(s);

    // s가 takes_ownership 함수의 인자로 이동되어 s는 유효하지않은 상태
    //println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn borrow() {
    let s = String::from("borrow");
    // & 키워드는 값을 참조하지만 소유하지는 않는 참조자를 생성하도록한다.
    // 소유권을 가지고 있지 않기 떄문에 넘겨받은 참조자를 이용해서 수정할수는 없다.
    let len = calculate_length(&s);

    println!("{}", len);

    // 컴파일 에러가 발생하지 않는다.
    println!("{}", s);
}

fn calculate_length(value: &String) -> usize {
    // `value` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // value.push_str("!!!");

    return value.len();
}

fn mut_reference() {
    let mut s = String::from("borrow");

    let r1 = &mut s;
    // second mutable borrow occurs here
    // 하나 이상의 mut레퍼런스를 빌려줄수 없다.
    // let r2 = &mut s;

    r1.push_str("!!!");

    println!("{}", s);
}

fn reference_limit() {
    let mut s = String::from("reference_limit");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; 

    r3.push_str("!!!");

    println!("{}", s);

    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // 가변 참조자가 존재하는 상태에서 불변참조자를 사용할 경우 에러를 뱉는다.
    // 불변참조자는 사용중인 동안 갑자기 값이 바뀔거라고 예상하지 않기 때문이다.
    // println!("{}", r1.len());
}

// 구조체 선언
// 구조체의 username과 email을 String 타입으로 지정하였기 때문에 
// username과 email에 할당되는 String은 해당 구조체 인스턴스가 유효한 동안만 유효하다.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_test() {
    let mut user = User {
        username: String::from("na yun su"),
        email: String::from("skennel2@gmail.com"),
        sign_in_count: 1, 
        active: true
    };

    let email_value = String::from("skennel2@gmail.com");
    //user 변수 지정시 mut로 되어있지 않다면 email속성을 변경하려는 동작에 대해 컴파일 에러가 발생한다.
    user.email = email_value;

    println!("{}", user.email);
}

fn mutabillity() {
    let user = User {
        username: String::from("na yun su"),
        email: String::from("skennel2@gmail.com"),
        sign_in_count: 1, 
        active: true
    };

    let email_value = String::from("skennel2@naver.com");

    // 변수가 mutable로 선언되어있지 않으면 변수 자체의 값이 아닌 변수가 가르키고 있는 대상의 어떤 값도 변경이 불가능하다.
    // user.email = email_value;

    println!("{}", user.email);
}

fn for_loop() {
    let count = 100;

    // 0 ~ 100 까지 루프
    let mut sum = 0;
    for n in 0..=count {
        sum += n;
    }

    println!("{}", sum);

    // 0 ~ 99 까지 루프
    let mut sum = 0;
    for n in 0..count {
        sum += n;
    }

    println!("{}", sum);
}

fn match_syntax() {
    let number = 14; 

    let result = match number % 2 {
        0 => true,
        1 => false,
        _ => false,
    };

    println!("{}", result);
}