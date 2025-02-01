



fn main() {
    let dog1 = Dog;
    let cat1 = Cat;

    //static dispatch
    //at compile time compiler knows which implementation of the animal trait to apply to each struct

    dog1.make_sound();
    cat1.make_sound();

    println!("-----");

    //dynamic dispatch(runtime polymorphism)

    perform_sound(&cat1);
    perform_sound(&dog1);

    //lets create a vector of animal trait structs

    // let mut v: Vec<Box<dyn Animal>> = Vec::new(), allows me to run v.iter()
    let mut v: Vec<dyn Animal> = Vec::new();
    v.push(cat1);
    v.push(dog1);

    println!("-----");


    for animal in v.iter(){
        animal.make_sound();
    }

    //messing around with vectors and indeces

    let s: String = String::from("Hello world general dynamics");

    let mut v: Vec<char> = Vec::new();

    for char in s.chars(){
        v.push(char);
    }
    
    for i in 0..v.len(){
        println!("{}",v[i]);
    }

}


trait Animal {
    fn make_sound(&self);
}

struct T;

struct Dog;

struct Cat;

impl Animal for Dog {
    fn make_sound(&self){
        println!("woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self){
        println!("meow");
    }
}
// just borrowing(reference)
fn perform_sound(animal: &dyn Animal){
    animal.make_sound();
}

