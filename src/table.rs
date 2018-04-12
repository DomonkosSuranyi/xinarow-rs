pub struct Table {
    fields: Vec<Vec<Option<Field>>>
}

#[derive(Clone, PartialEq, Debug)]
pub enum Field {
    X,
    O,
}

impl Table {
    pub fn new(size: (usize,usize)) -> Self {
        Table { fields: vec![vec![None; size.1]; size.0] }
    }

    pub fn set(&mut self, pos: (usize,usize), field: Field) -> () {
        self.fields[pos.0][pos.1] = Some(field);
    }
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
}
