pub mod hosting;

pub struct Breakfast {
    pub toast: String, // need to make the struct and required feild pub
    seasonal_fruit: String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches")
        }
    }
}