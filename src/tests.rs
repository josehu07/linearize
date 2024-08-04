//! Tests for the linearizer.

use super::*;

#[test]
fn is_linearizable() {
    let mut linearizer = Linearizer::new(3);
    let node_ops: [(Node, OpSpan); 10] = [
        (0, OpSpan::new(Some(8), None, (100, 105))),
        (1, OpSpan::new(Some(7), None, (104, 106))),
        (2, OpSpan::new(None, Some(7), (102, 108))),
        (1, OpSpan::new(None, Some(8), (110, 112))),
        (2, OpSpan::new(None, Some(9), (109, 115))),
        (0, OpSpan::new(None, Some(8), (110, 117))),
        (1, OpSpan::new(Some(9), None, (114, 118))),
        (2, OpSpan::terminate()),
        (1, OpSpan::terminate()),
        (0, OpSpan::terminate()),
    ];
    for (node, span) in node_ops {
        assert!(linearizer.feed_span(node, span));
    }
}

#[test]
#[should_panic]
fn detect_violation() {
    let mut linearizer = Linearizer::new(3);
    let node_ops: [(Node, OpSpan); 7] = [
        (0, OpSpan::new(Some(8), None, (100, 105))),
        (1, OpSpan::new(Some(7), None, (104, 106))),
        (2, OpSpan::new(Some(9), None, (107, 110))),
        (0, OpSpan::new(None, Some(7), (111, 113))),
        (2, OpSpan::terminate()),
        (0, OpSpan::terminate()),
        (1, OpSpan::terminate()),
    ];
    for (node, span) in node_ops.into_iter() {
        assert!(linearizer.feed_span(node, span));
    }
}

#[test]
fn no_partial_stepped() {
    let mut linearizer = Linearizer::new(2);
    let node_ops: [(Node, OpSpan); 8] = [
        (0, OpSpan::new(Some(7), None, (100, 106))),
        (1, OpSpan::new(Some(7), None, (104, 105))),
        (0, OpSpan::new(Some(7), None, (110, 116))),
        (1, OpSpan::new(Some(7), None, (114, 115))),
        (0, OpSpan::new(Some(7), None, (120, 126))),
        (1, OpSpan::new(Some(7), None, (124, 125))),
        (0, OpSpan::terminate()),
        (1, OpSpan::terminate()),
    ];
    for (node, span) in node_ops {
        assert!(linearizer.feed_span(node, span));
        for possibility in &linearizer.possibilities {
            assert!(!possibility.can_step());
        }
    }
}
