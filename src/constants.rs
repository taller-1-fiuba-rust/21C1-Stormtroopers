/** Data base types **/
pub static TYPE_STRING: &str = "String";
pub static TYPE_LIST: &str = "List";
pub static TYPE_SET: &str = "Set";

/** Constants used in the app **/
pub static THREAD_POOL_COUNT: usize = 8;
pub static END_FLAG: &str = "exit";
pub static MSG_OVER: &str = "MESSAGE: Connection over\n";
pub static LINE_BREAK: char = '\n';
pub static RESP_SIMPLE_STRING: &str = "OK\r\n";
pub const ERROR_LOG_CREATE: &str = "Error creating Logger";
pub const ERROR_DBFILE_CREATE: &str = "Error creating database dump file";
pub const DBDUMP_INTERVAL_SECS: u64 = 10;
pub const DBDUMP_PATH: &str = "./";
pub const EMPTY_STR: &str = "";
