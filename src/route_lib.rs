use odbc_api::{buffers::{BufferDesc, ColumnarAnyBuffer}, ConnectionOptions, Environment, Cursor};
use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
pub async fn test_conn() -> String {
    let env = match Environment::new() {
        Ok(environment) => environment,
        Err(_) => {
            return String::from("ERROR: ENVIRONMENT NOT CREATED");
        }
    };

    let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";

    let conn = env.connect_with_connection_string(conn_str, ConnectionOptions::default());

    match conn {
        Ok(_) => String::from("CONNECTION CREATED"),
        Err(_) => String::from("ERROR: CONNECTION NOT ESTABLISHED"),
    }
}

#[get("/")]
pub async fn test_read() -> String {
    let batch_size = 1000;
    let buffer_desc = [
        BufferDesc::I32 { nullable: false },
        BufferDesc::Text { max_str_len: 255 },
    ];
    let mut buffer = ColumnarAnyBuffer::from_descs(batch_size, buffer_desc);

    let env = Environment::new().expect("FAILED TO CREATE ENV");
    let conn_str = "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=SampleDatabase;Trusted_Connection=yes;";
    let to_return = match env.connect_with_connection_string(conn_str, ConnectionOptions::default()) {
        Ok(connection) => {
            match connection.execute("select * from ProductTbl", ()) {
                Ok(option_cursor) => {
                    match option_cursor {
                        Some(cursor) => {
                            match cursor.bind_buffer(&mut buffer) {
                                Ok(mut row_set_cursor) => {
                                    match row_set_cursor.fetch() {
                                        Ok(option_row_set) => {
                                            if let Some(row_set) = option_row_set {
                                                let col1 = row_set.column(0);
                                                match col1.as_slice::<i32>() {
                                                    Some(a) => {
                                                        let mut str = String::new();
                                                        for b in a {
                                                            str.push_str(format!("{b} \n").as_str())
                                                        }
                                                        str
                                                    },
                                                    None => "ERROR".to_owned()
                                                }
                                            } else {
                                                "ERROR".to_owned()
                                            }
                                        },
                                        Err(_) => "ERROR".to_owned()
                                    }
                                },
                                Err(_) => "ERROR".to_owned()
                            }
                        },
                        None => "ERROR".to_owned()
                    }
                },
                Err(_) => "ERROR".to_owned()
            }
        },
        Err(_) => "ERROR".to_owned(),
    };
    to_return
}

#[get("/dyn_param/<something>")]
pub async fn test_create(something: i32) -> i32 {
    something
}