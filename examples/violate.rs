//! Example of linearizability violation.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
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
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }
}
