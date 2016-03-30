//! Library description
#![deny(missing_docs, unsafe_code)]
pub mod types;
pub mod worker;
// pub mod supervisor;

// pub use self::types::{Message};
// pub use self::worker::Worker;
// pub use self::supervisor::Supervisor;

#[macro_use]
extern crate log;
extern crate env_logger;
