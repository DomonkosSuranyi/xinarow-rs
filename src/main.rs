pub mod table;
use table::{Table};

fn main() {
    let table_size = (5,3);
    let mut table = Table::new(table_size);
    println!("{}", table);
}

