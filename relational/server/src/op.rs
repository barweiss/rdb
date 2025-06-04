use crate::row::Row;

pub trait Op {
    fn execute(&self) -> impl Iterator<Item = dyn Row>;
}
