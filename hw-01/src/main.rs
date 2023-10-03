fn main() {
    let text_1: String = "Hello ".to_string();
    let text_2: &str = "World";
    let text_3: char = '!';

    print_any(&text_1);
    print_any(text_2);
    print_any(&text_3);
}

fn print_any<T: std::fmt::Display + ?Sized>(text: &T) {
    print!("{}", text);
}