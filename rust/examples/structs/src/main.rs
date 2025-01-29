
pub struct Car {

    pub year: String,
    pub model: String,
    pub num_of_engines: u8,
    pub mph: u8,
}

pub struct Boat {
    pub year: String,
    pub model: String,
    pub num_of_engines: u8,
    pub knots: u8,
}

pub trait Describe {
    fn summarize(&self) -> String;
}
impl Describe for Car {
    fn summarize(&self) -> String {
        return self.year.clone();
    }
}

impl Describe for Boat{
    fn summarize(&self) -> String {
        return self.model.clone();
    }
}
//traits contain fn
fn main() {
    let car1 = Car {
        year: String::from("1950"),
        model: String::from("Ford"),
        num_of_engines: 1,
        mph: 50,
    };

    let boat1 = Boat {
        year: String::from("2000"),
        model: String::from("navy"),
        num_of_engines: 2,
        knots: 20,
    };

    println!("{}",car1.summarize());
    println!("{}",boat1.summarize());
}
