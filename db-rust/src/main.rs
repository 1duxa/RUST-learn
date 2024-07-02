use table::{ColumnType, ColumnValue, Table};

mod database;
mod table;



fn main() {

    let types = vec![ColumnType::Integer,ColumnType::Varchar];
    let mut table_a = Table::new(types);
    let _ = table_a.add_row(vec![ColumnValue::Integer(34),ColumnValue::Varchar("AAA".to_string())]);
    let _ = table_a.add_row(vec![ColumnValue::Integer(67),ColumnValue::Varchar("ASDDDD".to_string())]);

    
    println!("{:?}",table_a.get_row(&0).expect("Nf"));
    dbg!(table_a);
}
