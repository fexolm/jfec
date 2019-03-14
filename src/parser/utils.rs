use pest::iterators::{Pair, Pairs};
use pest::RuleType;
use std::io;
pub fn to_string<R: RuleType>(p: Pair<R>) -> String {
    p.as_str().to_string()
}

pub fn next_string<R: RuleType>(p: &mut Pairs<R>) -> Result<String, io::Error> {
    let next = get_next(p)?;
    Ok(to_string(next))
}

pub fn get_next<'s, 't, R: RuleType>(p: &'s mut Pairs<'t, R>) -> Result<Pair<'t, R>, io::Error> {
    if let Some(val) = p.next() {
        Ok(val)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "kek"))
    }
}

pub fn inner_next<R: RuleType>(p: Pair<R>) -> Result<Pair<R>, io::Error> {
    get_next(&mut p.into_inner())
}