use std :: io;
use rand::Rng;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
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

fn generate_workout(marks: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(5));
        num
    });

    if marks <= 15 {
        println!("Today, do {} hours of study", expensive_result.value(marks));
        // println!("Next, do {} hours of study", expensive_result.value(marks));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to revise your work");
        } else {
            println!(
                "Today, study for {} minutes!",
                expensive_result.value(marks)
            );
        }
    }
}

fn main() {
    println!("Enter marks obtained");
    let  mut simulated_user_specified_value =String::new();
    io::stdin()
            .read_line(&mut simulated_user_specified_value)
            .expect("Failed to read line");

        let uservalue: u32 = simulated_user_specified_value.trim().parse().expect("not a number") ;
    

    let simulated_random_number = rand::thread_rng().gen_range(1..10);
    println!("Random number {}",simulated_random_number);

    generate_workout(uservalue, simulated_random_number);
}
