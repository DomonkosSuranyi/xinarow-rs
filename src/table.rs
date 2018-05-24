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

    pub fn row_len(&self) -> usize {
        self.fields.len()
    }
}

use std::fmt;

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut outp = "".to_owned();
        add_line(self.fields.len(), &mut outp);
        for row in 0..self.fields[0].len() {
            outp.push_str("\n |");
            for col in 0..self.fields.len() {
                outp.push_str(" ");
                outp.push_str(self.fields[col][row].value());
            }
            outp.push_str(" |");
        }
        outp.push_str("\n");
        add_line(self.fields.len(), &mut outp);
        write!(f, "{}", outp)
    }
}

fn add_line(length: usize, s: &mut String) {
    s.push_str(" +");
    for col in 0..length {
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
