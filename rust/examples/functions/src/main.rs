fn main() {
    let x: u32 = 5;
    let y: u32 = 5;

    let st = "this land was made for you and me";
    print_shit(x);

    println!("x*y is: {}",mult(x,y));
    println!("split string is: {}",stringIndexValue(st.to_string(),6));
}

//print method but must take u32
fn print_shit(x: u32){
    println!("x is: {}",x);
}

fn mult(x: u32,y: u32) -> u32 {
    return &x * &y;
}


//function to print value of string at index, i.e its char
fn stringIndexValue(a: String,i:usize) -> String {
    let s = a.chars().nth(i).unwrap();
    return String::from(s);
}
