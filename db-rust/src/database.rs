use std::collections::{HashMap, HashSet};

use anyhow::Error;

use crate::table::Table;



pub struct Database {
    name:String,
    tables:HashMap<String,Table>
}


impl Database {
    
    pub fn new(db_name:&str) -> Self{
        Self { name: db_name.to_string(), tables: HashMap::new() }
    }
    pub fn add_table(&mut self,name:&str,table:Table) -> Result<(),Error>{

        self.tables.insert(name.to_string(),table);

        Ok(())
    }
}
