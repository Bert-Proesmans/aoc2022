pub fn most_calories_carrying_elf<IteratorType, IntermediateType, ItemType>(
    it: IteratorType,
) -> Option<i64>
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

pub fn top_n_calories_carrying_elf<IteratorType, IntermediateType, ItemType>(
    it: IteratorType,
    nth: usize,
) -> i64
where
    IteratorType: Iterator<Item = IntermediateType>,
    IntermediateType: IntoIterator<Item = ItemType>,
    ItemType: AsRef<str>,
{
    let mut calories: Vec<_> = it
        .flat_map(|x| x.into_iter().map(|y| y.as_ref().parse::<i32>()))
        .scan(0, |acc, e| -> Option<Option<i64>> {
            let mut yield_value = Some(None);
            match e {
                Err(_) => {
                    yield_value = Some(Some(*acc));
                    *acc = 0;
                }
                Ok(value) => *acc += value as i64,
            };
            yield_value
        })
        // Funky time required because returning a None from iter:scan stops the iterator
        // So we pad the iterator with None's when values are accumulated
        .flatten()
        .collect();
    calories.sort_unstable_by(|a, b| b.cmp(a)); // sort High -> Low
    calories.iter().take(nth).sum()
}
