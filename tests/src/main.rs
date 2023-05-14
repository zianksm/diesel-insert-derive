use diesel::Insertable;


#[diesel_insert_derive::auto_insert(table_name = some_table)]
pub struct TestTable {
    id: i64,
}

fn main() {
    println!("Testing");
}
