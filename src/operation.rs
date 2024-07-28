//! Definition of a Put/Get operation span with start-end timestamps.

use std::fmt;

/// Value type.
pub type Value = u64;

/// Node ID type; each node is e.g. a server in a cluster.
/// Currently assumes node IDs start from 0 (so can be used directly as index).
pub type Node = usize;

/// Timestamp type; expected to be monotonically increasing and always unique.
pub type Timestamp = u64;

/// Operation inputs enum.
#[derive(Debug, Clone)]
pub enum OpInputs {
    Put { val: Value },
    Get,
}

/// Operation result enum.
#[derive(Debug, Clone)]
pub enum OpResult {
    Put,
    Get {
        val: Option<Value>, // `None` if not found
    },
}

/// An operation span from a node with start-end timestamps.
#[derive(Clone)]
pub struct OpSpan {
    pub inputs: OpInputs,
    pub result: OpResult,
    pub ts_req: Timestamp,
    pub ts_ack: Timestamp,
}

impl OpSpan {
    /// Convenience constructor for `OpSpan`.
    pub fn new(
        val_i: Option<Value>,            // input value; `None` means Get, else Put
        val_o: Option<Value>,            // result value
        ts_span: (Timestamp, Timestamp), // (ts_req, ts_ack)
    ) -> Self {
        let (ts_req, ts_ack) = ts_span;
        assert!(ts_ack > ts_req);

        let inputs = if let Some(val) = val_i {
            OpInputs::Put { val }
        } else {
            OpInputs::Get
        };
        let result = if val_i.is_some() {
            OpResult::Put
        } else {
            OpResult::Get { val: val_o }
        };

        OpSpan {
            inputs,
            result,
            ts_req,
            ts_ack,
        }
    }

    /// Special constructor for an `OpSpan` that indicates termination of a
    /// node's execution, by assigning an "infinite" timestamp.
    pub fn terminate() -> Self {
        OpSpan {
            inputs: OpInputs::Get,
            result: OpResult::Put,
            ts_req: Timestamp::MAX,
            ts_ack: Timestamp::MAX,
        }
    }

    /// Check if an `OpSpan` is termination.
    pub fn is_terminate(&self) -> bool {
        self.ts_req == Timestamp::MAX
    }
}

impl fmt::Debug for OpSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}<{}>-<{}>",
            if let OpInputs::Put { val } = self.inputs {
                format!("Put({})", val)
            } else {
                "Get".into()
            },
            if let OpResult::Get { val } = self.result {
                if let Some(val) = val {
                    format!("({})", val)
                } else {
                    "(nil)".into()
                }
            } else {
                "".into()
            },
            if self.is_terminate() {
                "X".into()
            } else {
                self.ts_req.to_string()
            },
            if self.is_terminate() {
                "X".into()
            } else {
                self.ts_ack.to_string()
            },
        )
    }
}

impl fmt::Display for OpSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if let OpInputs::Put { val } = self.inputs {
                format!("Put({})", val)
            } else {
                "Get".into()
            },
            if let OpResult::Get { val } = self.result {
                if let Some(val) = val {
                    format!("({})", val)
                } else {
                    "(nil)".into()
                }
            } else {
                "".into()
            },
        )
    }
}
