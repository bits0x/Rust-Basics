use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    //------------------------------------------

    let x: Vec<i32> = vec![1, 2, 3];

    let equal_to_x = move |z: Vec<i32>| z == x;

    // println!("can't use x here: {:?}", x); // as move, moves the ownership

    let y: Vec<i32> = vec![1, 2, 3];

    assert!(equal_to_x(y));

}

struct Cacher<T> 
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

 
fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cached_result.value(intensity));
        println!("Today, do {} situps", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take abreak toady, Remember keep hydrates");
        }
        else {
            println!(
                "Today run for {} minutes",
                cached_result.value(intensity) 
            );
        }
    }
}