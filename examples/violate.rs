//! Example of linearizability violation.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
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
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }
}
