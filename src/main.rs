mod custom_types;

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
        job_title: "Software Developer".to_string(),
    };

    println!("{:?}", me);
}
