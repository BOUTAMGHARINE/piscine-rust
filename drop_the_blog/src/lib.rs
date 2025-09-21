use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Blog {
    pub fn new() -> Blog {}
    pub fn new_article(&self, body: String) -> (usize, Article) {}
    pub fn new_id(&self) -> usize {}
    pub fn is_dropped(&self, id: usize) -> bool {}
    pub fn add_drop(&self, id: usize) {}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Article {}
    pub fn discard(self) {}
}