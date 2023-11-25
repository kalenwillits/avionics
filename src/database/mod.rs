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
    query += "ID TEXT";
    for (i, column) in table.columns.iter().enumerate() {
        query += format!("{} {}", column.name, "").as_str();
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
    pub fn def(&self) -> String {
        let mut typedef: String;
        match self.datatype {
            DataType::Null => {
                typedef = "NULL".to_string();
            }
            DataType::Integer => {
                typedef = "INTEGER".to_string();
            }
            DataType::Real => {
                typedef = "REAL".to_string();
            }
            DataType::Text => {
                typedef = "TEXT".to_string();
            }
            DataType::Blob => {
                typedef = "BLOB".to_string();
            }
        };
        format!("{} {} NOT NULL", self.name, typedef)
    }
}

pub enum DataType {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}
