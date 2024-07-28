//! A possible state that stores the current value and per-node queues of
//! not-yet processed operations.

use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::hash;

use crate::{Node, OpInputs, OpResult, OpSpan, Value};

/// A possible state linearized upto the current point.
///
/// TODO: one potential performance optimization is to have a global list of
///       queues (instead of per-state copies of queues), and let each state
///       store indices/pointers to which op in each node's queue is its next
///       head; the lineage history, if needed, can be stored similarly
#[derive(Debug, Clone)]
pub(crate) struct Possibility {
    /// Current body value; `None` if empty.
    current_body: Option<Value>,

    /// Linear history of operations applied that led to `current_body`.
    lineage_history: Vec<(Node, OpSpan)>,

    /// Node-indexed queues of operations to be checked from that node.
    queued_spans: Vec<VecDeque<OpSpan>>,
}

impl Possibility {
    /// Make an initial empty state with null value.
    pub(crate) fn initial(num_nodes: usize) -> Self {
        Possibility {
            current_body: None,
            lineage_history: vec![],
            queued_spans: (0..num_nodes).map(|_| VecDeque::new()).collect(),
        }
    }

    /// Add a new span to its corresponding queue.
    pub(crate) fn append_span(&mut self, node: Node, span: OpSpan) {
        debug_assert!(node < self.queued_spans.len());
        debug_assert!((span.ts_ack > span.ts_req) || (span.is_terminate()));
        if let Some(head) = self.queued_spans[node].front() {
            // for every node, its submitted operations must naturally follow
            // a sequential order already
            assert!(span.ts_req > head.ts_ack);
        }

        self.queued_spans[node].push_back(span);
    }

    /// Check if I can consume myself and make a step into further state(s).
    pub(crate) fn can_step(&self) -> bool {
        // === have seen at least 1 op from every node
        (!self.queued_spans.iter().any(|q| q.is_empty()))
            && (!self
                .queued_spans
                .iter()
                .all(|q| q.front().unwrap().is_terminate()))
    }

    /// Consume myself and step into 0-to-some further possible state(s). The
    /// resulting states might still be steppable.
    pub(crate) fn step(self) -> HashSet<Self> {
        assert!(self.can_step());
        let mut new_states = HashSet::new();

        let min_ts_ack = self
            .queued_spans
            .iter()
            .map(|q| q.front().unwrap().ts_ack)
            .min()
            .unwrap();
        for (node, q) in self.queued_spans.iter().enumerate() {
            if q.front().unwrap().ts_req < min_ts_ack {
                // possible candidate as the next op
                if let Some(new_state) = self.apply_head(node as Node) {
                    new_states.insert(new_state);
                }
            }
        }

        new_states
    }

    /// Attempt to apply the head operation on given node's queue as the next
    /// operation, returning a valid copy of state on success or a `None` on
    /// error or value mismatch.
    fn apply_head(&self, node: Node) -> Option<Self> {
        let op = self.queued_spans[node].front().unwrap();

        match op.inputs {
            OpInputs::Put { val } => {
                match op.result {
                    OpResult::Put => {
                        // successful Put
                        let mut new_state = self.clone();
                        new_state
                            .lineage_history
                            .push((node, new_state.queued_spans[node].pop_front().unwrap()));
                        new_state.current_body = Some(val);
                        Some(new_state)
                    }
                    _ => {
                        // Put with unexpected result
                        None
                    }
                }
            }

            OpInputs::Get => {
                match op.result {
                    OpResult::Get { val } => {
                        if self.current_body == val {
                            // successful Get with matching value
                            let mut new_state = self.clone();
                            new_state
                                .lineage_history
                                .push((node, new_state.queued_spans[node].pop_front().unwrap()));
                            Some(new_state)
                        } else {
                            // successful Get but values mismatch
                            None
                        }
                    }
                    _ => {
                        // Get with unexpected result
                        None
                    }
                }
            }
        }
    }
}

impl fmt::Display for Possibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}<|[",
            if let Some(val) = self.current_body {
                val.to_string()
            } else {
                "nil".into()
            }
        )?;
        for (i, q) in self.queued_spans.iter().enumerate() {
            write!(f, "{}", q.len())?;
            if i < self.queued_spans.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]~")?;
        for (i, (n, s)) in self.lineage_history.iter().enumerate() {
            write!(f, "{}-{}", n, s)?;
            if i < self.lineage_history.len() - 1 {
                write!(f, "~")?;
            }
        }
        Ok(())
    }
}

impl cmp::PartialEq for Possibility {
    fn eq(&self, other: &Self) -> bool {
        self.current_body == other.current_body
            && self.queued_spans.len() == other.queued_spans.len()
            && self
                .queued_spans
                .iter()
                .map(|q| q.len())
                .zip(other.queued_spans.iter().map(|q| q.len()))
                // comparing length of each node's queue is enough when
                // determining equality between possibilities
                .all(|(ls, lo)| ls == lo)
    }
}

impl cmp::Eq for Possibility {}

impl hash::Hash for Possibility {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.current_body.hash(state);
        self.queued_spans.len().hash(state);
        for q in &self.queued_spans {
            q.len().hash(state);
        }
    }
}
