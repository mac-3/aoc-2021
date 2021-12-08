use std::collections::{HashMap, HashSet};

use crate::digit::Digit;

#[derive(Debug)]
pub struct State<'a> {
    map: HashMap<char, HashSet<char>>,
    unsolved: Vec<&'a str>,
}

impl<'a> State<'a> {
    pub fn init() -> State<'a> {
        let mut map = HashMap::with_capacity(7);
        ('a'..='g').for_each(|c| {
            map.insert(c, ('a'..='g').collect::<HashSet<char>>());
        });
        State {
            map,
            unsolved: Vec::with_capacity(6),
        }
    }

    pub fn reduce(&mut self, digit: Digit, chars: &HashSet<char>) {
        assert_eq!(digit.chars_hashset().len(), chars.len());
        let diff = Digit(8)
            .chars_hashset()
            .difference(&digit.chars_hashset())
            .cloned()
            .collect::<HashSet<char>>();
        chars.iter().for_each(|c| {
            diff.iter().for_each(|d| {
                self.map.get_mut(c).unwrap().remove(d);
            })
        })
    }

    pub fn resolve(self) -> HashMap<char, char> {
        let mut acc = HashMap::with_capacity(7);
        self.resolve_recv(0, &mut HashSet::new(), &mut acc);
        acc
    }

    fn resolve_recv(
        &self,
        cursor: usize,
        used: &mut HashSet<char>,
        assoc: &mut HashMap<char, char>,
    ) -> bool {
        if cursor >= 7 {
            let len = self.unsolved.len();
            let parsed_len = self
                .unsolved
                .iter()
                .filter_map(|s| Digit::from_str_mapping(s, assoc))
                .collect::<HashSet<Digit>>()
                .len();
            return len == parsed_len;
        }
        const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        for c in self.map.get(&CHARS[cursor]).unwrap() {
            if !used.contains(c) {
                used.insert(*c);
                assoc.insert(CHARS[cursor], *c);
                if self.resolve_recv(cursor + 1, used, assoc) {
                    return true;
                }
                assoc.remove(&CHARS[cursor]);
                used.remove(c);
            }
        }
        false
    }
}

pub fn part_b(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut sep = l.split('|');
            (sep.next().unwrap(), sep.next().unwrap())
        })
        .map(|(left, right)| parse_display(right, find_assoc(left)))
        .sum::<usize>()
}

fn find_assoc(input: &str) -> HashMap<char, char> {
    let mut state = State::init();
    input.split(' ').filter(|s| !s.is_empty()).for_each(|s| {
        let possible_digits = Digit::from_len(s.chars().count());
        match possible_digits.len() {
            1 => state.reduce(possible_digits[0], &s.chars().collect::<HashSet<char>>()),
            _ => state.unsolved.push(s),
        }
    });
    state.resolve()
}

fn parse_display(input: &str, assoc: HashMap<char, char>) -> usize {
    let mut acc = 0usize;
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .filter_map(|d| Digit::from_str_mapping(d, &assoc))
        .for_each(|d| {
            acc = acc * 10 + d.0 as usize;
        });
    acc
}
