fn main() {
    let x: u32 = 5;
    let y: u32 = 5;

    print_shit(x);

    println!("x*y is: {}",mult(x,y));
}

fn print_shit(x: u32){
    println!("x is: {}",x);
}

fn mult(x: u32,y: u32) -> u32 {
    return &x * &y;
}
