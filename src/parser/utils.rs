use pest::iterators::{Pair, Pairs};
use pest::RuleType;

pub fn to_string<R: RuleType>(p: Pair<R>) -> String {
    p.as_str().to_string()
}

pub fn next_string<R: RuleType>(p: &mut Pairs<R>) -> String {
    to_string(p.next().unwrap())
}

pub fn next_iter<R: RuleType>(p: Pair<R>) -> Pair<R> {
    p.into_inner().next().unwrap()
}