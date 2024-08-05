//! Example of complex checks with failed ops and node resumes.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
    let mut linearizer = Linearizer::new(3);

    let node_ops: [(Node, OpSpan); 15] = [
        (2, OpSpan::get(None, 99, 101)),
        (0, OpSpan::put(8, 100, 105)),
        (1, OpSpan::put(7, 104, 106)),
        (2, OpSpan::get(Some(7), 102, 108)),
        (1, OpSpan::get(Some(8), 110, 112)),
        (2, OpSpan::get(Some(9), 109, 115)),
        (1, OpSpan::fail(114, 118)),
        (0, OpSpan::get(Some(10), 117, 119)),
        (2, OpSpan::put(11, 120, 123)),
        (2, OpSpan::stopped(125)),
        (1, OpSpan::put(12, 124, 127)),
        (1, OpSpan::stopped(128)),
        (0, OpSpan::stopped(129)),
        (2, OpSpan::resumed(130)),
        (2, OpSpan::get(Some(11), 131, 132)),
    ];

    for (node, span) in node_ops.into_iter() {
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }
}
