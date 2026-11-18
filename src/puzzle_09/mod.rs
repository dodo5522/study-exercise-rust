use std::{cmp::Ordering, io::Error};

pub fn sort_float<T>(target: Vec<T>) -> Result<Vec<T>, Error>
where
    T: PartialOrd + PartialEq,
{
    let mut sorted = target;
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    Ok(sorted)
}
