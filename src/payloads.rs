
/// Saves the payload in a static variable
pub static DETECTIONS: &[&str] = &[
    "'",
    "' --",
    "' -- ",
    "'#",
];

/// Saves keywords to detect if the form is vulnerable to sql injection
pub static KEYWORDS_DETECTION: &[&str] = &[
    "error",
    "syntax",
    "mysql",
    "sql",
    "exception",
    "warning",
    "postgres",
    "sqlite",
    "oracle",
    "db2",
    "mssql",
    "access",
    "odbc",
    "dbi",
    "rdbms",
    "rdb",
    "newsql",
    "new sql",
    "new-sql",
    "fatal",
];