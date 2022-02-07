mod modtest;

fn main() {
    print_hello_world();
    string_type();
    copy_value();
    move_memory();
    ownership();
    borrow();
    mut_reference();
    reference_limit();
    struct_test();

    println!("{}", modtest::get_hello_world());
}

fn print_hello_world() {
    let value = "Hello World";

    println!("{}", value);
}

fn string_type() {
    let literal_string = "hello string literal";
    let mut string_type = String::from("Hello String");
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

    let emailValue = String::from("skennel2@gmail.com");
    //user 변수 지정시 mut로 되어있지 않다면 email속성을 변경하려는 동작에 대해 컴파일 에러가 발생한다.
    user.email = emailValue;

    println!("{}", user.email);
}