use std::{collections::HashSet, io};

use progr::prelude::*;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    // let item = Item::new(
    //     "john",
    //     HashSet::from([
    //         "human".into(),
    //         "garfield owner".into(),
    //         "invalid\ntag".into(),
    //     ]),
    //     "John Arbuckle",
    // );
    // item.write()?;

    //let mut item = Item::open("john")?;
    //item.tags.insert("male".into());
    //item.write()?;

    //println!("{:?}", item);

    let stage = Stage::new(
        "todo",
        HashSet::from(["player-movement".into(), "player-hud".into()]),
    );
    stage.write()?;

    let stage = Stage::open("todo").unwrap();
    stage.write()?;

    println!("{:?}", stage);

    Ok(())
}
