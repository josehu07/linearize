mod operation;
pub use operation::{Node, OpInputs, OpResult, OpSpan, Timestamp, Value};

mod linearizer;
pub use linearizer::Linearizer;

mod possibility;
use possibility::Possibility;

#[cfg(test)]
mod tests;
