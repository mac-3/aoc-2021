use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digit(pub u8);

impl Digit {
    pub fn from_str(input: &str) -> Option<Digit> {
        let mut array = [false; 7];
        input.chars().for_each(|c| {
            array[c as usize - 'a' as usize] = true;
        });
        match array {
            [true, true, true, false, true, true, true] => Some(Digit(0)),
            [false, false, true, false, false, true, false] => Some(Digit(1)),
            [true, false, true, true, true, false, true] => Some(Digit(2)),
            [true, false, true, true, false, true, true] => Some(Digit(3)),
            [false, true, true, true, false, true, false] => Some(Digit(4)),
            [true, true, false, true, false, true, true] => Some(Digit(5)),
            [true, true, false, true, true, true, true] => Some(Digit(6)),
            [true, false, true, false, false, true, false] => Some(Digit(7)),
            [true, true, true, true, true, true, true] => Some(Digit(8)),
            [true, true, true, true, false, true, true] => Some(Digit(9)),
            _ => None,
        }
    }

    pub fn from_str_mapping(input: &str, mapping: &HashMap<char, char>) -> Option<Digit> {
        let mapped = input
            .chars()
            .map(|c| *mapping.get(&c).unwrap())
            .collect::<String>();
        Digit::from_str(&mapped)
    }

    pub fn chars_hashset(&self) -> HashSet<char> {
        match self.0 {
            0 => HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']),
            1 => HashSet::from(['c', 'f']),
            2 => HashSet::from(['a', 'c', 'd', 'e', 'g']),
            3 => HashSet::from(['a', 'c', 'd', 'f', 'g']),
            4 => HashSet::from(['b', 'c', 'd', 'f']),
            5 => HashSet::from(['a', 'b', 'd', 'f', 'g']),
            6 => HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']),
            7 => HashSet::from(['a', 'c', 'f']),
            8 => HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']),
            9 => HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']),
            _ => unreachable!(),
        }
    }

    pub fn from_len(len: usize) -> Vec<Digit> {
        match len {
            2 => vec![Digit(1)],
            3 => vec![Digit(7)],
            4 => vec![Digit(4)],
            5 => vec![Digit(2), Digit(3), Digit(5)],
            6 => vec![Digit(0), Digit(6), Digit(9)],
            7 => vec![Digit(8)],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars_to_digit() {
        assert_eq!(Digit::from_str("abcefg").unwrap().0, 0);
        assert_eq!(Digit::from_str("cf").unwrap().0, 1);
        assert_eq!(Digit::from_str("acdeg").unwrap().0, 2);
        assert_eq!(Digit::from_str("acdfg").unwrap().0, 3);
        assert_eq!(Digit::from_str("bcdf").unwrap().0, 4);
        assert_eq!(Digit::from_str("abdfg").unwrap().0, 5);
        assert_eq!(Digit::from_str("abdefg").unwrap().0, 6);
        assert_eq!(Digit::from_str("acf").unwrap().0, 7);
        assert_eq!(Digit::from_str("abcdefg").unwrap().0, 8);
        assert_eq!(Digit::from_str("abcdfg").unwrap().0, 9);
    }
}
