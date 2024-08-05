//! Tests for the linearizer.

use super::*;

#[test]
fn is_linearizable() {
    let mut linearizer = Linearizer::new(3);
    let node_ops: [(Node, OpSpan); 11] = [
        (2, OpSpan::get(None, 99, 101)),
        (0, OpSpan::put(8, 100, 105)),
        (1, OpSpan::put(7, 104, 106)),
        (2, OpSpan::get(Some(7), 102, 108)),
        (1, OpSpan::get(Some(8), 110, 112)),
        (2, OpSpan::get(Some(9), 109, 115)),
        (0, OpSpan::get(Some(8), 111, 117)),
        (1, OpSpan::put(9, 114, 118)),
        (2, OpSpan::stopped(116)),
        (1, OpSpan::stopped(120)),
        (0, OpSpan::stopped(119)),
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
        (0, OpSpan::put(8, 100, 105)),
        (1, OpSpan::put(7, 104, 106)),
        (2, OpSpan::put(9, 107, 110)),
        (0, OpSpan::get(Some(7), 111, 113)),
        (2, OpSpan::stopped(112)),
        (0, OpSpan::stopped(115)),
        (1, OpSpan::stopped(114)),
    ];
    for (node, span) in node_ops.into_iter() {
        assert!(linearizer.feed_span(node, span));
    }
}

#[test]
fn with_failed_ops() {
    let mut linearizer = Linearizer::new(2);
    let node_ops: [(Node, OpSpan); 9] = [
        (0, OpSpan::put(8, 100, 105)),
        (1, OpSpan::fail(104, 107)),
        (1, OpSpan::get(Some(8), 109, 110)),
        (0, OpSpan::get(Some(7), 111, 112)),
        (1, OpSpan::get(Some(9), 113, 114)),
        (1, OpSpan::put(10, 115, 117)),
        (0, OpSpan::get(Some(10), 118, 119)),
        (1, OpSpan::stopped(120)),
        (0, OpSpan::stopped(121)),
    ];
    for (node, span) in node_ops {
        assert!(linearizer.feed_span(node, span));
    }
}

#[test]
#[should_panic]
fn with_stop_resume() {
    let mut linearizer = Linearizer::new(2);
    let node_ops: [(Node, OpSpan); 8] = [
        (0, OpSpan::put(8, 100, 105)),
        (1, OpSpan::put(7, 104, 106)),
        (1, OpSpan::stopped(107)),
        (0, OpSpan::get(Some(7), 108, 110)),
        (0, OpSpan::put(9, 111, 114)),
        (0, OpSpan::stopped(115)),
        (1, OpSpan::resumed(117)),
        (1, OpSpan::get(Some(7), 118, 120)),
    ];
    for (node, span) in node_ops.into_iter() {
        assert!(linearizer.feed_span(node, span));
    }
}

#[test]
fn no_partial_stepped() {
    let mut linearizer = Linearizer::new(2);
    let node_ops: [(Node, OpSpan); 8] = [
        (0, OpSpan::put(7, 100, 106)),
        (1, OpSpan::put(7, 104, 105)),
        (0, OpSpan::put(7, 110, 116)),
        (1, OpSpan::put(7, 114, 115)),
        (0, OpSpan::put(7, 120, 126)),
        (1, OpSpan::put(7, 124, 125)),
        (0, OpSpan::stopped(130)),
        (1, OpSpan::stopped(131)),
    ];
    for (node, span) in node_ops {
        assert!(linearizer.feed_span(node, span));
        for possibility in &linearizer.possibilities {
            assert!(!possibility.can_step());
        }
    }
}
