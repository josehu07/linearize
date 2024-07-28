//! Example of successful checks.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
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

    for (node, span) in node_ops.into_iter() {
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }
}
