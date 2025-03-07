pub trait PullOp {
    type Row: crate::storage::Row;

    fn pull(&self) -> impl Iterator<Item = Self::Row>;
}

pub trait TransformOp {
    type Row: crate::storage::Row;

    fn transform(&self, input: &impl Iterator<Item = Self::Row>)
    -> impl Iterator<Item = Self::Row>;
}

pub struct OpNode<Row: crate::storage::Row> {
    child: Box<OpNode<Row>>,
}
