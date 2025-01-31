//! Definition of a Put/Get operation span with start-end timestamps.

use std::fmt;

/// Value type.
pub type Value = u64;

/// Node ID type; each node is e.g. a server in a cluster.
/// Currently assumes node IDs start from 0 (so can be used directly as index).
pub type Node = usize;

/// Timestamp type; expected to be **monotonically increasing** and **always
/// unique**.
pub type Timestamp = u64;

/// Operation inputs enum.
#[derive(Debug, Clone)]
pub enum OpInputs {
    Put { val: Value },
    Get,
    Fail,    // leaves value uncertain
    Stopped, // indicates node temporarily stopped
    Resumed, // indicates node execution resumed
}

/// Operation result enum.
#[derive(Debug, Clone)]
pub enum OpResult {
    Put,
    Get {
        val: Option<Value>, // `None` if not found
    },
    Dummy,
}

/// An operation span with start-end timestamps.
#[derive(Clone)]
pub struct OpSpan {
    pub(crate) inputs: OpInputs,
    pub(crate) result: OpResult,
    pub(crate) ts_req: Timestamp,
    pub(crate) ts_ack: Timestamp,
}

impl OpSpan {
    /// Create an `OpSpan` for a successful Put operation.
    pub fn put(val_i: Value, ts_req: Timestamp, ts_ack: Timestamp) -> Self {
        assert!(ts_ack > ts_req);
        OpSpan {
            inputs: OpInputs::Put { val: val_i },
            result: OpResult::Put,
            ts_req,
            ts_ack,
        }
    }

    /// Create an `OpSpan` for a successful Get operation.
    pub fn get(val_o: Option<Value>, ts_req: Timestamp, ts_ack: Timestamp) -> Self {
        assert!(ts_ack > ts_req);
        OpSpan {
            inputs: OpInputs::Get,
            result: OpResult::Get { val: val_o },
            ts_req,
            ts_ack,
        }
    }

    /// Create an `OpSpan` for a failed operation, leaving value uncertain.
    pub fn fail(ts_req: Timestamp, ts_ack: Timestamp) -> Self {
        assert!(ts_ack > ts_req);
        OpSpan {
            inputs: OpInputs::Fail,
            result: OpResult::Dummy,
            ts_req,
            ts_ack,
        }
    }

    /// Special constructor for an `OpSpan` that indicates stopping of a
    /// node's execution.
    pub fn stopped(ts: Timestamp) -> Self {
        OpSpan {
            inputs: OpInputs::Stopped,
            result: OpResult::Dummy,
            ts_req: ts,
            ts_ack: ts,
        }
    }

    /// Special constructor for an `OpSpan` that indicates resuming of a
    /// node's execution.
    pub fn resumed(ts: Timestamp) -> Self {
        OpSpan {
            inputs: OpInputs::Resumed,
            result: OpResult::Dummy,
            ts_req: ts,
            ts_ack: ts,
        }
    }

    /// Check if an `OpSpan` is a normal operation.
    pub fn is_normal(&self) -> bool {
        matches!(
            self.inputs,
            OpInputs::Put { .. } | OpInputs::Get | OpInputs::Fail
        )
    }
}

impl fmt::Debug for OpSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}<{}>-<{}>", self, self.ts_req, self.ts_ack,)
    }
}

impl fmt::Display for OpSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.inputs {
                OpInputs::Put { val } => format!("Put({})", val),
                OpInputs::Get => "Get".into(),
                OpInputs::Fail => "Fail".into(),
                OpInputs::Stopped => "Stopped".into(),
                OpInputs::Resumed => "Resumed".into(),
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
