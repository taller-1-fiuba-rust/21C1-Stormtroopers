//! Some of the common constants used in various crates.
/** Data base types **/
pub static TYPE_STRING: &str = "String";
pub static TYPE_LIST: &str = "List";
pub static TYPE_SET: &str = "Set";

/** Constants used in the app **/
pub static THREAD_POOL_COUNT: usize = 20; //ponerlo en el config para que se pueda cambiar
pub static END_FLAG: &str = "exit";
pub static MSG_OVER: &str = "MESSAGE: Connection over\n";
pub static LINE_BREAK: char = '\n';
pub static RESPONSE_SIMPLE_STRING: &str = "OK\r\n";
pub const ERROR_LOG_CREATE: &str = "Error creating Logger";
pub const ERROR_DBFILE_CREATE: &str = "Error creating database dump file";
pub const DBDUMP_INTERVAL_SECS: u64 = 600;
pub const DBDUMP_PATH: &str = "./";
pub const MARK_BULLET: &str = ") ";
pub const NIL_RESPONSE: &str = "(nil)\n";
pub const TTL_CHECK_RANGE: u64 = 30;
pub const TTL_SLEEP_TIME: u64 = 10;

/** Default values **/
pub static SHARDING_COUNT_DEFAULT: u32 = 4;
