use super::super::types::Person;

pub fn handle_job_title(person: &Person) -> () {
    match &person.job_title {
        Some(title) => println!("{} is a {}", person.name, title),
        None => println!("{} doesn't have a job", person.name),
    }
}
