mod custom_types;

use custom_types::Color;
use custom_types::Person;

fn main() {
    let _an_int: i32 = 32;
    let _a_string: &str = "Hello, world!";
    let _a_bool: bool = true;
    let _an_array: [i32; 3] = [1, 2, 3];

    let me: Person = Person {
        name: "Matthew Pagan".to_string(),
        age: 32,
        hobby: "Coding".to_string(),
        job_title: Some("Software Developer".to_string()),
        favorite_color: Color::Cyan,
    };

    let noah: Person = Person {
        name: "Noah Pagan".to_string(),
        age: 5,
        hobby: "Lego".to_string(),
        job_title: None,
        favorite_color: Color::Blue,
    };

    match me.job_title {
        Some(title) => println!("{} is a {}", me.name, title),
        None => println!("{} doesn't have a job", me.name),
    }

    match noah.job_title {
        Some(title) => println!("{} is a {}", noah.name, title),
        None => println!("{} doesn't have a job", noah.name),
    }
}
