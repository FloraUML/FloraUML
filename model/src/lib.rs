#[derive(Debug, PartialEq)]
pub struct Model<'a> {
    pub classes: Vec<&'a str>,
}
