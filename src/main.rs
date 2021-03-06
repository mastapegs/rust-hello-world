mod types;
use types::{Color, Person};

mod utils;
use utils::handle_job_title;

fn main() {
    let people: [Person; 2] = [
        Person {
            name: "Matthew Pagan".to_string(),
            age: 32,
            hobby: "Coding".to_string(),
            job_title: Some("Software Developer".to_string()),
            favorite_color: Color::Cyan,
        },
        Person {
            name: "Noah Pagan".to_string(),
            age: 5,
            hobby: "Lego".to_string(),
            job_title: None,
            favorite_color: Color::Blue,
        },
    ];

    for person in people.iter() {
        handle_job_title(person);
    }

    fn add_numbers(x: i32, y: i32) -> i32 {
        x + y
    }

    let x = 3;

    println!("{} + {} = {}", 1, x, add_numbers(1, x));
    println!("{} + {} = {}", 1, x, add_numbers(5, x));

    fn print_string(x: &String) {
        println!("{}", x);
    }

    let name = String::from("Matthew Pagan");
    print_string(&name);
    print_string(&name);
}
