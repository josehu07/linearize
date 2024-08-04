//! Simple on-line linearizability checker.

use std::collections::HashSet;
use std::fmt;
use std::mem;

use crate::{Node, OpSpan, Possibility};

/// On-line per-object linearizability checker.
#[derive(Debug, Clone)]
pub struct Linearizer {
    /// Number of nodes.
    pub(crate) num_nodes: usize,

    /// Collection of currently possible correct states.
    pub(crate) possibilities: HashSet<Possibility>,
}

impl Linearizer {
    /// Create a new linearizer with just one empty initial state to start with.
    pub fn new(num_nodes: usize) -> Self {
        assert_ne!(num_nodes, 0);
        Linearizer {
            num_nodes,
            possibilities: HashSet::from([Possibility::initial(num_nodes)]),
        }
    }

    /// Feed in a new operation span to all the current possible states, and
    /// may trigger them to step into further state(s).
    ///
    /// Returns true if still have possibilities left after stepping attempt;
    /// otherwise returns false, meaning linearizability has been violated.
    pub fn feed_span(&mut self, node: Node, span: OpSpan) -> bool {
        assert!(node < self.num_nodes);
        assert!((span.ts_ack > span.ts_req) || (span.is_terminate()));

        if self.possibilities.is_empty() {
            // already violated, always return false
            return false;
        }

        // append the new span to all current states, then put all steppable
        // states into `pending` and others into `self.possibilities`
        let mut pending = HashSet::new();
        let mut new_pending = HashSet::new();
        for possibility in self.possibilities.drain().map(|mut p| {
            p.append_span(node, span.clone());
            p
        }) {
            if possibility.can_step() {
                pending.insert(possibility);
            } else {
                new_pending.insert(possibility);
            }
        }
        mem::swap(&mut self.possibilities, &mut new_pending);

        // recursively call `.step()` until no pending states left
        //
        // TODO: for each feed, the loop below obviously has the potential to
        //       be parallelized to boost checker performance
        while !pending.is_empty() {
            for possibility in pending.drain() {
                debug_assert!(possibility.can_step());
                for new_possibility in possibility.step() {
                    if new_possibility.can_step() {
                        new_pending.insert(new_possibility);
                    } else {
                        self.possibilities.insert(new_possibility);
                    }
                }
            }
            mem::swap(&mut pending, &mut new_pending);
        }

        !self.possibilities.is_empty()
    }
}

impl fmt::Display for Linearizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Possibilities {{")?;
        for possibility in &self.possibilities {
            writeln!(f, "  {}", possibility)?;
        }
        write!(f, "}}")
    }
}
