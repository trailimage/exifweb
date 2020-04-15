#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CategoryKind {
    Who,
    When,
    Where,
    What,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub kind: CategoryKind,
}
