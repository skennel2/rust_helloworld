fn main() {
    print_hello_world();
    string_type();
    copy_value();
    move_memory();
    ownership();
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
