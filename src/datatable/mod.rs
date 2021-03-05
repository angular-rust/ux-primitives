use intmap::IntMap;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Channel<'a> {
    pub name: &'a str,
    pub tag: u8,
    pub visible: bool,
}

#[derive(Clone, Debug)]
pub struct DataFrame<M, D>
where
    M: fmt::Display,
{
    // metric contain something like a timestamp, or month names
    pub metric: M,
    // data key is tag for stream meta (a.k.a column tag)
    // D is
    pub data: IntMap<D>,
}
