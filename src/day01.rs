pub fn most_calories_carrying_elf<IteratorType, IntermediateType, ItemType>(it: IteratorType) -> Option<i64>
where
    IteratorType: Iterator<Item = IntermediateType>,
    IntermediateType: IntoIterator<Item = ItemType>,
    ItemType: AsRef<str>,
{
    it.map(|x| x.into_iter().map(|x| x.as_ref().parse::<i32>()))
        .flatten()
        .fold((None, 0), |(acc, state), e| match e {
            Err(_) => match acc {
                Some(highest_value) if highest_value < state => (Some(state), 0),
                Some(_highest_value) => (acc, 0),
                None => (Some(state), 0),
            },
            Ok(value) => (acc, state + value as i64),
        })
        .0
}
