use crate::row::Row;

type RowIter = Iter<Item = Row>;

pub trait Op {
    execute(&self) -> Iter<Item = Row>
}

struct OpTree {
    root: OpNode,
}
