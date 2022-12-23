pub struct Unsee<Item, T>
where
    T: Iterator<Item = Item>,
{
    stack: Vec<Item>,
    iter: T,
}

impl<Item, T> Unsee<Item, T>
where
    T: Iterator<Item = Item>,
{
    pub fn wrap(iter: T) -> Unsee<Item, T> {
        Unsee {
            stack: vec![],
            iter,
        }
    }

    pub fn unsee(&mut self, s: Item) {
        self.stack.push(s);
    }
}

impl<Item, T> Iterator for Unsee<Item, T>
where
    T: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        match self.stack.pop() {
            None => self.iter.next(),
            Some(s) => Some(s),
        }
    }
}
