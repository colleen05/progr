use std::{collections::HashSet, io};

use progr::prelude::*;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let ws = Workspace::open()?;
    println!("{:?}", ws);
    //ws.write_all()?;

    Ok(())
}
