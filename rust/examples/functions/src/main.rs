
mod test;
//mod testdir;

fn main() {
    let x: u32 = 5;
    let y: u32 = 5;

    let st = "this land was made for you and me";
    print_shit(x);

    //println!("x*y is: {}",mult(x,y));
    println!("split string is: {}",stringIndexValue(st.to_string(),6));
    loopString(st.to_string());
    test::function();
    println!("-------");

    

   
}

//print method but must take u32
fn print_shit(x: u32){
    println!("x is: {}",x);
}
fn mult(x: u32,y: u32) -> u32 {
    return &x * &y;
}

//function to print value of string at index, i.e it chars
fn stringIndexValue(a: String,i:usize) -> String {
    let s = a.chars().nth(i).unwrap();
    return String::from(s);
}

fn loopString(a: String){
    for i in 0..a.chars().count(){
        if a.chars().nth(i).unwrap() == char::from('e'){
            println!("FOUND e");
        }
        println!("i is : {}",String::from(a.chars().nth(i).unwrap()));
    }
}