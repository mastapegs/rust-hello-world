use super::Color;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub hobby: String,
    pub job_title: String,
    pub favorite_color: Color,
}