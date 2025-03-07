use sqlparser::ast::Statement;

pub trait Planner {
    type Plan: super::Plan;

    fn plan(&self, ast: Vec<Statement>) -> Self::Plan;
}
