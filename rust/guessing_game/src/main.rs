use std::io;
use rand::Rng;

fn main() {
    //println!("Guess a number");
    println!("Input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}",secret_number);
    //let guess = String::new() : this creates a immutable string variable(unchangeable)
    let mut guess = String::new(); // : this creates a mutable string variable(changeable)


    // here the read_line function returns a Result value, an enum with two possible values(variants)
    // Result value can be either Ok or Err :
    //      Ok indicates a succesful operation and the generated value
    //      Err indicates the operation failed and contains information about the failure

    // The Result type has methods defined to them; an instance of Resule has an expect method you can call
    //      the expect method will crash if Result is an Err value.
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}",guess);    

    //io::stdout().flush();
    println!("Second test");

    let x = 5;
    let y = 10;

    println!("x = {} and y + 2 = {}",x,y+2);

}
