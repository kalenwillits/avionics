use crate::database::{Column, DataType, Table};

pub const CONFIG: Table<Column<DataType>, 2> = Table {
    name: "CONFIG",
    columns: [
        Column {
            name: "AIRCRAFT",
            datatype: DataType::Integer,
        },
        Column {
            name: "PILOT",
            datatype: DataType::Integer,
        },
    ],
};

pub const AIRCRAFT: Table<Column<DataType>, 3> = Table {
    name: "AIRCRAFT",
    columns: [
        Column {
            name: "CALLSIGN",
            datatype: DataType::Text,
        },
        Column {
            name: "MAKE",
            datatype: DataType::Text,
        },
        Column {
            name: "MODEL",
            datatype: DataType::Text,
        },
    ],
};

pub const ENGINE: Table<Column<DataType>, 7> = Table {
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
        Column {
            name: "NORMAL_OPERATING_MIN",
            datatype: DataType::Integer,
        },
        Column {
            name: "NORMAL_OPERATING_MAX",
            datatype: DataType::Integer,
        },
        Column {
            name: "AIRCRAFT",
            datatype: DataType::Integer,
        },
    ],
};

pub const PILOT: Table<Column<DataType>, 1> = Table {
    name: "PILOT",
    columns: [Column {
        name: "NAME",
        datatype: DataType::Text,
    }],
};
