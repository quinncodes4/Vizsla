fn main() {
    
    
    //empty string variable
    let s1 = String::new();

    //creating a string with a value
    let s2 = String::from("helloWorld");


    let s2 = s2 + &String::from("ok");

    //b is of type unsigned int 8 bit
    let b: u32 = 90;

    //c is a floating point 8 bit value
    let mut c: f32 = 3.0;

    //numeric operations

    let z: u32;

    //multipying two integers
    //let z = &x + &b;

    //println!("x is : {}",x);
    println!("s1 is : {}",s1);
    println!("s2 is : {}",s2);
    //println!("b is : {}",b);
    println!("c is : {}",c);
    //println!("z is : {}",z);

    let x: u32 = 5;

    println!("test function return value is : {}",test_function(b,x));

    //test_function(b);

}
fn test_function(x: u32, y: u32) -> u32
{
    return x * y;
}
