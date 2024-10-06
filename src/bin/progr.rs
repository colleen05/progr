use std::{collections::HashSet, io};

use progr::item::Item;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let item = Item::new(
        "john",
        HashSet::from([
            "human".into(),
            "garfield owner".into(),
            "invalid\ntag".into(),
        ]),
        "John Arbuckle",
    );
    item.write()?;

    let mut item = Item::open("john")?;
    item.tags.insert("male".into());
    item.write()?;

    println!("{:?}", item);

    Ok(())
}
