use std::{collections::HashSet, io};

use progr::prelude::*;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let tag = Tag::open("player");

    println!("{:?}", tag);

    Ok(())
}
