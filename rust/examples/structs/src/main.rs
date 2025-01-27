
struct Engine {
    fuel_type: String,
    horsepower: u32,
    weight: u32,
}

fn main() {
    let mut e1 = Engine{
        fuel_type: String::from("Gasoline"),
        horsepower: 50,
        weight: 500
    };


    println!("Engine horsepower is: {}",e1.horsepower);

    e1.horsepower = 80;

    println!("Engine horsepower is: {}",e1.horsepower);

    let mut e2 = build_engine(String::from("Diesel"),300,900);

    println!("Engine 2 horsepower is: {}",e2.horsepower);
    println!("Engine 2 fuel type is: {}",e2.fuel_type);
}
//create and return an instance of struct

//note: if just assigning value dont need the object reference, but if multiplying we do
fn build_engine(s: String,h: u32,w: u32) -> Engine {
    Engine{
        fuel_type: s,
        horsepower: &h * &w,
        weight: w,
    }
}
