use std::{collections::HashMap, ops::Index};

use crate::value::Value;

pub trait Row<'a> {
    fn value(self, name: &str) -> Option<&'a Value>;
    fn value_at(self, index: usize) -> &'a Value;
    fn column_name(self, index: usize) -> Option<&'a str>;
}

impl<'a> Index<usize> for dyn Row<'a> {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        self.value(index)
    }
}

impl<'a> Index<&str> for dyn Row<'a> {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        self.value(index)
    }
}

pub struct MemRow<'a> {
    values_by_name: HashMap<&'a str, Value>,
    column_order: Vec<String>,
}

impl Row for MemRow {
    fn value(self, index: usize) -> &Value {
        self.value(self.column_name(index))
    }

    fn value(self, name: &str) -> Option<&Value> {
        self.values_by_name.get(value)
    }

    fn column_name(self, index: usize) -> Option<&str> {
        self.column_order[index]
    }
}

impl MemRow {
    fn from_map(values_by_name: HashMap<&str, Value>, column_order: Vec<String>) -> MemRow {
        if values_by_name.len() != column_order.len() {
            panic!(
                "mem row length mismatch: {} values and {} columns",
                values_by_name.len(),
                column_order.len()
            )
        }

        MemRow {
            values_by_name,
            column_order,
        }
    }

    fn from_pairs(values: &mut [(String, Value)]) {
        let mut values_by_name = HashMap::with_capacity(values.len());
        let mut column_order = Vec::with_capacity(values.len());
        for (name, value) in values {
            column_order.push(name);
            values_by_name.insert(column_order.last().unwrap().as_str(), v)
        }

        Self::from_map(values_by_name, column_order)
    }
}
