use diesel_insert_derive::exclude;


diesel::table! {
    accounts (id) {
        id -> Int8,
        name -> Text,
        phone -> Varchar,
        password -> Varchar,
    }
}
#[diesel_insert_derive::auto_insert(table_name = accounts)]
#[exclude(id)]
pub struct TestTable {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub password: String,
}



fn main() {
    println!("Testing");
}
