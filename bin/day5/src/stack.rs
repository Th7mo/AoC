#[derive(Clone)]
pub struct Stack {
    items: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn take(&mut self, amount: u8, together: bool) -> Self {
        let mut taken_items = Vec::new();

        for _ in 0..amount {
            let Some(latest_item) = self.items.pop() else {
                panic!("The stack you wanted to take items from is already empty!");
            };
            taken_items.push(latest_item)
        }

        if together {
            taken_items.reverse();
        }

        Self { items: taken_items }
    }

    pub fn add_new(&mut self, item: char) {
        self.items.push(item);
    }

    pub fn add(&mut self, mut other: Self) {
        self.items.append(&mut other.items);
    }

    pub fn last(&self) -> char {
        *self.items.last().expect("stack is empty")
    }
}
