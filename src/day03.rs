use itermore::prelude::*;

fn backpack_duplicate_items(it: impl IntoIterator<Item = String>) -> impl Iterator<Item = char> {
    it.into_iter().flat_map(|items| {
        if items.len() < 1 {
            return None;
        }

        let first_half = items[..items.len() / 2].chars();
        let second_half = &items[items.len() / 2..];
        for item in first_half {
            if second_half.contains([item]) {
                return Some(item);
            }
        }

        return None;
    })
}

fn backpack_common_badges(it: impl IntoIterator<Item = String>) -> impl Iterator<Item = char> {
    it.into_iter()
    .filter(|x| x.len() > 0)
    .array_chunks::<3>()
    .flat_map(|[first, second, third]| {
        let first = first.chars();
        for badge in first {
            if second.contains([badge]) && third.contains([badge]) {
                return Some(badge);
            }
        }

        return None;
    })
}

fn calculate_item_priorities(it: impl IntoIterator<Item = char>) -> usize {
    it.into_iter()
        .map(|item| match item {
            'a'..='z' => item as u32 - 'a' as u32 + 1,
            'A'..='Z' => item as u32 - 'A' as u32 + 27,
            _ => 0,
        } as usize)
        .sum()
}

pub fn backpack_duplicate_item_priorities(it: impl IntoIterator<Item = String>) -> usize {
    calculate_item_priorities(backpack_duplicate_items(it))
}

pub fn backpack_calculate_badge_priorities(it: impl IntoIterator<Item = String>) -> usize {
    calculate_item_priorities(backpack_common_badges(it))
}
