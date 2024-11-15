use std::fmt::Display;
mod select;
pub use select::*;

use sqlx::{query_builder::Separated, Database, Encode, Postgres, Type};
pub struct FunctionCallColumn<C> {
    pub function_name: &'static str,
    pub column: C,
}
pub trait ColumnType {
    fn column_name(&self) -> &'static str;

    fn format_column_with_prefix(&self, prefix: &str) -> String {
        format!("{}.{}", prefix, self.column_name())
    }
    fn all() -> Vec<Self>
    where
        Self: Sized;

    fn lower(&self) -> FunctionCallColumn<Self>
    where
        Self: Sized + Copy,
    {
        FunctionCallColumn {
            function_name: "LOWER",
            column: *self,
        }
    }
    fn upper(&self) -> FunctionCallColumn<Self>
    where
        Self: Sized + Copy,
    {
        FunctionCallColumn {
            function_name: "UPPER",
            column: *self,
        }
    }
}
pub fn concat_columns<T>(columns: &[T], prefix: Option<&str>) -> String
where
    T: ColumnType,
{
    if let Some(prefix) = prefix {
        columns
            .iter()
            .map(|column| column.format_column_with_prefix(prefix))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        columns
            .iter()
            .map(|column| column.column_name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub trait SeparatedExt<'args, DB>
where
    DB: Database,
{
    fn like_lower_and_bind(&mut self, column: impl ColumnType, value: &str);
    fn column_equals<T>(&mut self, column: impl ColumnType, value: T)
    where
        T: 'args + Encode<'args, DB> + Type<DB>;
}

impl<'args> SeparatedExt<'args, Postgres> for Separated<'_, 'args, Postgres, &str> {
    fn like_lower_and_bind(&mut self, column: impl ColumnType, value: &str) {
        self.push(format!("LOWER({}) LIKE ", column.column_name()));
        self.push_bind_unseparated(format!("%{}%", value.to_lowercase()));
    }

    fn column_equals<T>(&mut self, column: impl ColumnType, value: T)
    where
        T: 'args + Encode<'args, Postgres> + Type<Postgres>,
    {
        self.push(format!("{} = ", column.column_name()));
        self.push_bind_unseparated(value);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AndOr {
    And,
    Or,
}
impl Display for AndOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhereComparison {
    Equals,
    Like,
}
impl Display for WhereComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Equals => write!(f, "="),
            Self::Like => write!(f, "LIKE"),
        }
    }
}
