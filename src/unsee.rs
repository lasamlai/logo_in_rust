pub struct Unsee<'a, Item> {
    stack: Vec<Item>,
    iter: Box<dyn Iterator<Item = Item> + 'a>,
}

impl<'a, Item> Unsee<'a, Item> {
    pub fn wrap(iter: impl Iterator<Item = Item> + 'a) -> Unsee<'a, Item> {
        Unsee {
            stack: vec![],
            iter: Box::new(iter),
        }
    }

    pub fn unsee(&mut self, s: Item) {
        self.stack.push(s);
    }
}

impl<'a, Item> Iterator for Unsee<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        match self.stack.pop() {
            None => self.iter.next(),
            Some(s) => Some(s),
        }
    }
}
