use std::{
    collections::HashMap,
    fmt
};

pub struct FlowMeta<'a> {
    pub name: &'a str,
    pub tag: u8,
    pub visible: bool
}

pub struct DataFrame<M, D>
where
    M: fmt::Display,
{
    // metric contain something like a timestamp, or month names
    pub metric: M,
    // data key is tag for stream meta (a.k.a column tag)
    // D is
    pub data: HashMap<u8, D>,
}
