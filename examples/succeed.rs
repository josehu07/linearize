//! Example of successful checks.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
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

    for (node, span) in node_ops.into_iter() {
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }
}
