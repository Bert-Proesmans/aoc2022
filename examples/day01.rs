use std::{cell::RefCell, rc::Rc};

//  Get-Content .\examples\day01.stdin | cargo run --example day01
fn main() {
    let clonable_iterator = std::io::stdin().lines().map(|e| e.map_err(|_| 0));
    let source = ReplaySubject::new(clonable_iterator);
    println!(
        "{:?}",
        aoc2022::day01::most_calories_carrying_elf(source.clone())
    );
    println!(
        "{:?}",
        aoc2022::day01::top_n_calories_carrying_elf(source.clone(), 3)
    );
}

#[derive(Debug)]
struct ReplaySubject<IteratorType, ItemType>
where
    IteratorType: Iterator<Item = ItemType>,
    ItemType: Clone,
{
    shared_state: Rc<RefCell<(IteratorType, Vec<ItemType>)>>,
    idx: usize,
}

impl<IteratorType, ItemType> ReplaySubject<IteratorType, ItemType>
where
    IteratorType: Iterator<Item = ItemType>,
    ItemType: Clone,
{
    fn new(iterator: IteratorType) -> Self {
        Self {
            shared_state: Rc::new(RefCell::new((iterator, vec![]))),
            idx: 0,
        }
    }
}

impl<IteratorType, ItemType> Clone for ReplaySubject<IteratorType, ItemType>
where
    IteratorType: Iterator<Item = ItemType>,
    ItemType: Clone,
{
    fn clone(&self) -> Self {
        Self { shared_state: self.shared_state.clone(), idx: self.idx.clone() }
    }
}

impl<IteratorType, ItemType> Iterator for ReplaySubject<IteratorType, ItemType>
where
    IteratorType: Iterator<Item = ItemType>,
    ItemType: Clone,
{
    type Item = ItemType;

    fn next(&mut self) -> Option<Self::Item> {
        let mut shared_state = self.shared_state.borrow_mut();
        let yielded_value = match shared_state.1.get(self.idx) {
            Some(value) => Some(value.clone()),
            None => match shared_state.0.next() {
                Some(next_value) => {
                    shared_state.1.push(next_value.clone());
                    Some(next_value.clone())
                }
                None => None,
            },
        };

        self.idx += 1;
        yielded_value
    }
}
