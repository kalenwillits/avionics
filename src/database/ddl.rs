use crate::database::{Column, DataType, Table};

pub const AIRCRAFT: Table<Column<DataType>, 1> = Table {
    name: "AIRCRAFT",
    columns: [Column {
        name: "CALLSIGN",
        datatype: DataType::Text,
    }],
};

pub const ENGINE: Table<Column<DataType>, 4> = Table {
    name: "ENGINE",
    columns: [
        Column {
            name: "MAKE",
            datatype: DataType::Text,
        },
        Column {
            name: "MODEL",
            datatype: DataType::Text,
        },
        Column {
            name: "RPM_MIN",
            datatype: DataType::Integer,
        },
        Column {
            name: "RPM_MAX",
            datatype: DataType::Integer,
        },
    ],
};
