use std::{collections::HashMap, default};
use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
pub enum ColumnType {
    Varchar,
    Integer,
    Float,
    Char,
    Bool,
}
#[derive(Debug)]
pub enum ColumnValue {

    Varchar(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Bool(bool),
}
type Columns = Vec<ColumnType>;
type ColumnValues = Vec<ColumnValue>;


pub struct Table {
    data: HashMap<u64, ColumnValues>,
    columns: Columns,
}

impl Table {
    pub fn new(columns: Vec<ColumnType>) -> Self {
        Self {
            data: HashMap::new(),
            columns,
        }
    }

    pub fn add_row(&mut self, values: ColumnValues) -> Result<(), Error> {
        if values.len() != self.columns.len() {
            return Err(Error::msg("Number of values does not match number of columns"));
        }

        let next_key = self.data.len() as u64;
        self.data.insert(next_key, values);

        Ok(())
    }
    pub fn get_row(&self,row:&u64) -> Result<&ColumnValues,Error> {
        match self.data.get(row) {
            Some(values) => Ok(values),
            None => Err(anyhow!("Not found"))
        }
    }
    pub fn get_columns(&self) -> &Vec<ColumnType> {
        &self.columns
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            columns: Vec::new(),
        }
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n  |")?;
        for (idx, column_type) in self.columns.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", column_type)?;
        }
        writeln!(f, "|")?;

        for (key, row) in &self.data {
            write!(f, "|{}| ", key)?;
            for (idx, value) in row.iter().enumerate() {
                if idx > 0 {
                    write!(f, ", ")?;
                }
                match value {
                    ColumnValue::Varchar(s) => write!(f, r#""{}""#, s)?,
                    ColumnValue::Integer(i) => write!(f, "{}", i)?,
                    ColumnValue::Float(fl) => write!(f, "{}", fl)?,
                    ColumnValue::Char(c) => write!(f, "'{}'", c)?,
                    ColumnValue::Bool(b) => write!(f, "{}", b)?,
                }
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}