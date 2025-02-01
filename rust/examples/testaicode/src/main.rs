fn print_individual_chars(input_str: &str) {
    for char in input_str.chars() {
        if char == char::from('e'){
            println!("FOUND e");
        }
        println!("{}", char);
    }
}

fn main() {
    let input_str = "Hello, World!";
    print_individual_chars(input_str);
}