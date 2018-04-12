pub mod table;
use table::{Table};

fn main() {
    let table_size = (5,3);
    let mut table = Table::new(table_size);
    print(table_size);
}

fn print(size: (usize, usize)) {
    print!("   |");
    for col in 0..size.1 {
        print!(" {0} |", col);
    }
    println!("");
    for row in 0..size.0 {
        print_line(size.1);
        print_numbered_line(row, size.1);
    }
}

fn print_line(num_of_cols: usize) {
    for col in 0..num_of_cols+1 {
        print!("---+");
    }
    println!("");
}

fn print_numbered_line(line_num: usize, num_of_cols: usize) {
    print!(" {0} |", line_num);
    for col in 0..num_of_cols {
        print!("   |");
    }
    println!("");
}
