# Linearize

This repo demonstrates a simple yet effective algorithm of an **on-line linearizability checker** for concurrent Put/Get operation results. The `linearize` Rust crate delivers an implementation of this algorithm.

## Definition

With multiple *nodes* issuing and completing concurrent *operations* on a single object, *linearizability* is defined as the conjunction of the following two conditions:

* there must be an equivalent global *sequential* order of all operations, where each operation observes the results of all preceding operations, and
* the global order must obey the *real-time* property: if an operation starts later than another one finishes (based on their timestamps), it must be placed after that one in the global order.

## Algorithm

TODO

## Usage

Build and run tests:

```
cargo test
```

Run demo examples:

```
cargo run --example succeed|violate
```
