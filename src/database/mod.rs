use sqlite;
pub mod resources;

mod ddl;

const DATABASE_FILE_NAME: &str = "data.db";

pub fn connect() -> resources::Database {
    let connection = sqlite::Connection::open_thread_safe(DATABASE_FILE_NAME)
        .expect("Unable to access data file!");
    init_table(ddl::AIRCRAFT, &connection);
    init_table(ddl::ENGINE, &connection);
    resources::Database { connection }
}

fn init_table<const COUNT: usize>(
    table: Table<Column<DataType>, COUNT>,
    connection: &sqlite::ConnectionThreadSafe,
) {
    let mut query: String = format!("CREATE TABLE IF NOT EXISTS {} ( ", table.name);
    query += "PK INTEGER NOT NULL PRIMARY KEY";
    for (i, column) in table.columns.iter().enumerate() {
        if i == 0 {
            query += ", "
        };
        query += format!("{} {}", column.name, column.typedef()).as_str();
        if (i + 1) < table.columns.len() {
            query += ", ";
        } else {
            query += " )";
        }
    }
    connection.execute(query).unwrap();
}

pub struct Table<T: Sized, const COUNT: usize> {
    name: &'static str,
    columns: [T; COUNT],
}

pub struct Column<T> {
    name: &'static str,
    datatype: T,
}

impl Column<DataType> {
    pub fn typedef(&self) -> String {
        let def: String;
        match self.datatype {
            DataType::Null => {
                def = "NULL".to_string();
            }
            DataType::Integer => {
                def = "INTEGER".to_string();
            }
            DataType::Real => {
                def = "REAL".to_string();
            }
            DataType::Text => {
                def = "TEXT".to_string();
            }
            DataType::Blob => {
                def = "BLOB".to_string();
            }
        };
        format!("{} NOT NULL", def)
    }
}

pub enum DataType {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}
