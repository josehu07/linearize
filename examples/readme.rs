//! Simple example presented in README.

use linearize::{Linearizer, Node, OpSpan};

fn main() {
    let mut linearizer = Linearizer::new(2);

    let node_ops: [(Node, OpSpan); 3] = [
        (0, OpSpan::put(55, 1, 5)),
        (1, OpSpan::put(66, 3, 6)),
        (1, OpSpan::get(Some(77), 10, 12)),
    ];

    for (node, span) in node_ops.into_iter() {
        println!();
        println!("Feed {} {:?}", node, span);
        let ok = linearizer.feed_span(node, span);
        println!("{} -> {}", linearizer, ok);
    }

    {
        let op_a = OpSpan::put(77, 7, 9);
        println!();
        println!("If n0 feeds {:?}:", op_a);
        println!();
        println!("Feed {} {:?}", 0, op_a);
        let mut linearizer = linearizer.clone();
        let ok = linearizer.feed_span(0, op_a);
        println!("{} -> {}", linearizer, ok);
    }

    {
        let op_b = OpSpan::put(77, 13, 14);
        println!();
        println!("If, instead, n0 feeds {:?}:", op_b);
        println!();
        println!("Feed {} {:?}", 0, op_b);
        let mut linearizer = linearizer.clone();
        let ok = linearizer.feed_span(0, op_b);
        println!("{} -> {}", linearizer, ok);
    }
}
