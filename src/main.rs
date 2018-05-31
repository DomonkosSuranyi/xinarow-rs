pub mod table;
use table::{Table};
use std::result::Result;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn main() -> GenResult<()> {
    println!("Width of the table: ");
    let width = read_usize_input()?;
    println!("height of the table: ");
    let height = read_usize_input()?;
    let mut table = Table::new((width, height));
    println!("{}", table);

    Ok(())
}

fn read_usize_input() -> GenResult<usize> {
    let mut value_in = String::new();
    std::io::stdin().read_line(&mut value_in)?;
    Ok(value_in.trim().parse::<usize>()?)
}

