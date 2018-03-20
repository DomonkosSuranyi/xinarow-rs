pub struct Table {
    fields: Vec<Vec<Field>>
}

#[derive(Clone, PartialEq, Debug)]
pub enum Field {
    X,
    O,
    Empty,
}

pub struct Pos {
    x: u8,
    y: u8,
}

impl Table {
    pub fn new(size: Pos) -> Self {
        Table { fields: vec![vec![Field::Empty; size.y as usize]; size.x as usize] }
    }
}

#[cfg(test)]
mod test {
    use super::Table;
    use super::Field;
    use super::Pos;

    #[test]
    fn new() {
        let table = Table::new(Pos { x:10, y:5 });
        for i in 0..10 {
            for j in 0..5 {
                assert_eq![table.fields[i][j], Field::Empty]
            }
        }
    }
}
