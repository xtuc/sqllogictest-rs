//! [Sqllogictest][Sqllogictest] parser and runner.
//!
//! [Sqllogictest]: https://www.sqlite.org/sqllogictest/doc/trunk/about.wiki
//!
//! # Usage
//!
//! Implement [`DB`] trait for your database structure:
//!
//! ```ignore
//! struct Database {...}
//!
//! impl sqllogictest::DB for Database {
//!     type Error = ...;
//!     fn run(&self, sql: &str) -> Result<String, Self::Error> {
//!         ...
//!     }
//! }
//! ```
//!
//! Create a [`Runner`] on your database instance, and then run the script:
//!
//! ```ignore
//! let mut tester = sqllogictest::Runner::new(Database::new());
//! let script = std::fs::read_to_string("script.slt").unwrap();
//! tester.run_script(&script);
//! ```
//!
//! You can also parse the script and execute the records separately:
//!
//! ```ignore
//! let records = sqllogictest::parse(&script).unwrap();
//! for record in records {
//!     tester.run(record);
//! }
//! ```

pub mod parser;
pub mod runner;

pub use self::parser::*;
pub use self::runner::*;

pub mod harness;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Query {
    loc: Location,
    conditions: Vec<Condition>,
    pub type_string: String,
    sort_mode: Option<SortMode>,
    label: Option<String>,
    /// The SQL command.
    pub sql: String,
    /// The expected results.
    expected_results: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    loc: Location,
    conditions: Vec<Condition>,
    /// The SQL command is expected to fail instead of to succeed.
    error: bool,
    /// The SQL command.
    pub sql: String,
    /// Expected rows affected.
    expected_count: Option<u64>,
}
