mod types;
use types::{Color, Person};

mod utils;
use utils::handle_job_title;

fn main() {
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

    handle_job_title(me);
    handle_job_title(noah);
}
