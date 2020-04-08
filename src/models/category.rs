use crate::models::Post;

pub enum CategoryKind {
    Who,
    When,
    Where,
    What,
}

pub struct Category<'a> {
    pub name: String,
    pub posts: Vec<&'a Post>,
    pub kind: CategoryKind,
}
