pub struct Table {
    fields: Vec<Vec<Field>>
}

#[derive(Clone, PartialEq, Debug)]
pub enum Field {
    X,
    O,
    Empty,
}

impl Field {
    fn value(&self) -> &str {
        match *self {
            Field::X => "X",
            Field::O => "O",
            Field::Empty => " "
        }
    }
}

impl Table {
    pub fn new(size: (usize,usize)) -> Self {
        Table { fields: vec![vec![Field::Empty; size.1]; size.0] }
    }

    pub fn set(&mut self, pos: (usize,usize), field: Field) -> () {
        self.fields[pos.0][pos.1] = field;
    }

    pub fn width(&self) -> usize {
        self.fields.len()
    }

    pub fn height(&self) -> usize {
        if self.width() == 0 { 0 } else { self.fields[0].len() }
    }
}

use std::fmt;
const A_ASCII: u8  = 97;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut outp = "".to_owned();
        let char_len: usize = (((self.height()-1) as f64).log10() + 1.0) as usize;
        for _space in 0..char_len+2 {
            outp.push(' ');
        }
        for col in 0..self.fields.len() {
            outp.push(' ');
            outp.push((A_ASCII+(col as u8)) as char);
        }
        outp.push('\n');
        add_line(self.fields.len(), &mut outp, char_len+1);
        for row in 0..self.fields[0].len() {
            outp.push('\n');
            let index_string = &row.to_string();
            outp.push_str(index_string);
            for _space in 0..(char_len - index_string.len()) {
                outp.push(' ');
            }
            outp.push_str(" |");
            for col in 0..self.fields.len() {
                outp.push_str(" ");
                outp.push_str(self.fields[col][row].value());
            }
            outp.push_str(" |");
        }
        outp.push_str("\n");
        add_line(self.fields.len(), &mut outp, char_len+1);
        write!(f, "{}", outp)
    }
}

fn add_line(length: usize, s: &mut String, indent: usize) {
    for _space in 0..indent {
        s.push(' ');
    }
    s.push('+');
    for _col in 0..length {
        s.push_str("--");
    }
    s.push_str("-+");
}

#[cfg(test)]
mod test {
    use super::Table;
    use super::Field;

    #[test]
    fn new() {
        let table = Table::new((10,5));
        for i in 0..10 {
            for j in 0..5 {
                assert_eq!(table.fields[i][j], None);
            }
        }
    }

    #[test]
    fn set() {
        let mut table = Table::new((1,1));
        assert_eq!(table.fields[0][0], None);
        table.set((0,0), Field::X);
        assert_eq!(table.fields[0][0], Some(Field::X));
    }
}
