use std::{collections::HashMap, ops::Index};

use crate::value::Value;

pub trait Row {
    fn value<'s>(&'s self, name: &str) -> Option<&'s Value>;
    fn value_at<'s>(&'s self, index: usize) -> &'s Value;
    fn column_name<'s>(&'s self, index: usize) -> &'s str;
}

impl Index<usize> for dyn Row {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        self.value_at(index)
    }
}

impl Index<&str> for dyn Row {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.value(index)
            .expect(format!("row does not have column {index}").as_str())
    }
}

pub struct MemRow<'a> {
    column_order: Vec<String>,
    values_by_name: HashMap<&str, Value>,
}

impl Row for MemRow<'_> {
    fn value_at<'s>(&'s self, index: usize) -> &'s Value {
        self.value(self.column_name(index))
            .expect("result of column_name must point to a valid value")
    }

    fn value<'s>(&'s self, name: &str) -> Option<&'s Value> {
        self.values_by_name.get(name)
    }

    fn column_name<'s>(&'s self, index: usize) -> &'s str {
        self.column_order[index].as_str()
    }
}

impl<'a> MemRow<'a> {
    fn from_pairs(values: impl ExactSizeIterator<Item = (String, Value)>) -> Self {
        let mut values_by_name = HashMap::with_capacity(values.len());
        let mut column_order = Vec::with_capacity(values.len());
        for (name, value) in values {
            column_order.push(name);
            values_by_name.insert(name.as_str(), value);
        }

        Self {
            values_by_name,
            column_order,
        }
    }
}
