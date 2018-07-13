pub mod table;
use table::{Table};
use std::result::Result;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn main() -> GenResult<()> {
    let width = read_until_valid("\nWidth of the table: ");
    let height = read_until_valid("\nHeight of the table: ");
    let mut table = Table::new((width, height));
    println!("{}", table);

    Ok(())
}

fn read_until_valid(text: &str) -> usize {
    println!("{}", text);
    match read_usize_input() {
        Ok(v) => v,
        Err(e)  => {
            println!("{}", "Please insert a valid number");
            read_until_valid(text)
        }
    }
}
fn read_usize_input() -> GenResult<usize> {
    let mut value_in = String::new();
    std::io::stdin().read_line(&mut value_in)?;
    Ok(value_in.trim().parse::<usize>()?)
}

