# Linearize

[![Format check](https://github.com/josehu07/linearize/actions/workflows/format.yml/badge.svg)](https://github.com/josehu07/linearize/actions?query=josehu07%3Aformat)
[![Build status](https://github.com/josehu07/linearize/actions/workflows/build.yml/badge.svg)](https://github.com/josehu07/linearize/actions?query=josehu07%3Abuild)
[![Tests status](https://github.com/josehu07/linearize/actions/workflows/tests.yml/badge.svg)](https://github.com/josehu07/linearize/actions?query=josehu07%3Atests)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This repo demonstrates a simple yet effective algorithm of an **on-line linearizability checker** for concurrent Put/Get operations by a known number of nodes. The algorithm is implemented as a `linearize` Rust crate.

## Linearizability

With multiple *nodes* issuing and completing concurrent *operations*, *linearizability* is defined as the conjunction of the following two conditions:

* there must exist an equivalent global *sequential* order of all operations, where each operation observes the results of all preceding operations, and
* the global order must obey the *real-time* property: if an operation starts later than another one finishes (based on their timestamps), it must be placed after that one in the global order.

For simplicity, we limit our discussion on a single object. The core idea behind this algorithm is to maintain a set of still-possible states (hereafter called *possibilities*) given the operation results observed. If this set ever becomes empty after feeding an operation result in, then we know linearizability has been violated. This shares similarities with model checking approaches such as in TLA⁺.

## Usage

Build and run tests:

```text
cargo test
```

Run demo examples:

```text
cargo run --example readme|succeed|violate|complex
```

See the documentation of publicly-exposed structs for more details.

## Algorithm

Click below for an explanation of the algorithm with a walk-through example.

<details>
<summary>Algorithm explanation with a walk-through example...</summary>

### Definitions

Each possibility is a "snapshot" of the object's value after successfully applying a sequence of operations. More precisely, a possibility tracks the following three things:

* `lineage`: the sequence of operations that have been applied; think of this as the determined prefix of a possible global sequential order
* `curr_val`: the current value of the object, obtained by starting from an initial nil value and applying the determined sequence
* `{node -> queue}`: per-node queues of operation results coming from each node which have not been applied yet

where each operation result denoted `<ts_req>Type(in/out)<ts_ack>` contains the following information besides its source node ID:

* `ts_req`: starting timestamp
* `Type(in/out)`: Put input/Get output
* `ts_ack`: finish timestamp

Let's assume all timestamps are unique, and operations from each node are always already in order (i.e., `ts_req` > its previous `ts_ack` fed).

Here is an example of a valid possibility, assuming a known number of 2 nodes `n0` and `n1`:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<1>Put(7)<4>` ~ `<3>Get(7)<6>`  |  7  |  n0 ➛ `<10>Get(8)<11>` ~ `<13>Put(9)<17>` </br> n1 ➛

</div>

### Walk-Through

The checker starts from an initial set that contains only one initial possibility.

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
&nbsp;  |  nil  |  n0 ➛ </br> n1 ➛

</div>

Nodes feed completed operations to the checker. For each operation fed, the checker pushes it to the back of the corresponding node's queue of every current possibility. Say node `n0` feeds a Put(55) that started on timestamp 1 and finished on 5:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
&nbsp;  |  nil  |  n0 ➛ `<1>Put(55)<5>` </br> n1 ➛

</div>

The checker tries to *step* each current possibility by consuming it, producing 0-to-some new possibilities, and adding them to the set. A possibility can be stepped iff. it has at least one pending operation from every node. Here, there's now only one possibility in the set and it cannot be stepped (as we don't yet know what the next op from `n1` would look like), so nothing happens.

Say `n1` then feeds a Put(66):

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
&nbsp;  |  nil  |  n0 ➛ `<1>Put(55)<5>` </br> n1 ➛ `<3>Put(66)<6>`

</div>

Now we know at least one operation from every node for this possibility, meaning it can be stepped. It picks candidate operations from heads of per-node queues and tries to apply the op to its current value; a successful apply produces a new possibility, while a Get with mismatching value produces none. In this case, either head is a valid candidate because their timestamp spans overlap and both are just Puts. After stepping, the possibility is consumed and two new valid possibilities are produced, so the set now looks like:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<1>Put(55)<5>`  |  55  |  n0 ➛ </br> n1 ➛ `<3>Put(66)<6>`
`<3>Put(66)<6>`  |  66  |  n0 ➛ `<1>Put(55)<5>` </br> n1 ➛

</div>

Stepping is attempted repeatedly until all possibilities in the new set cannot be stepped.

Say `n1` then feeds a Get(77) that started late:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<1>Put(55)<5>`  |  55  |  n0 ➛ </br> n1 ➛ `<3>Put(66)<6>` ~ `<10>Get(77)<12>`
`<3>Put(66)<6>`  |  66  |  n0 ➛ `<1>Put(55)<5>` </br> n1 ➛ `<10>Get(77)<12>`

</div>

while this may look like a linearizability violation at first glance, we can't say for sure because `n0` could have made a Put(77) sometime in the middle. Anyways, feeding this Get makes the second possibility steppable; but this time, only the Put(55) can be a valid next operation. The Get(77) cannot be chosen as a candidate because of two reasons: 1. it started strictly after the finish of Put(55), and 2. even if it overlapped with the Put, its output does not match the current value 66. The new set after stepping:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<1>Put(55)<5>`  |  55  |  n0 ➛ </br> n1 ➛ `<3>Put(66)<6>` ~ `<10>Get(77)<12>`
`<3>Put(66)<6>` ~ `<1>Put(55)<5>`  |  55  |  n0 ➛ </br> n1 ➛ `<10>Get(77)<12>`

</div>

Say `n0` then feeds a Put(77) which indeed happened in the middle:

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<1>Put(55)<5>`  |  55  |  n0 ➛ `<7>Put(77)<9>` </br> n1 ➛ `<3>Put(66)<6>` ~ `<10>Get(77)<12>`
`<3>Put(66)<6>` ~ `<1>Put(55)<5>`  |  55  |  n0 ➛ `<7>Put(77)<9>` </br> n1 ➛ `<10>Get(77)<12>`

</div>

After stepping all current possibilities exhaustively, the set reduces to one possibility, and linearizability still holds. Note that operations Put(66) and Put(55) are swappable in the lineage history, but we consider both as the same possibility as they don't affect the checker's decisions beyond.

<div align="center">

lineage history | current value | per-node queues
:-- | :-: | :--
`<3>Put(66)<6>` ~ `<1>Put(55)<5>` ~ `<7>Put(77)<9>`  |  77  |  n0 ➛ </br> n1 ➛ `<10>Get(77)<12>`

</div>

Consider, alternatively, that `n0` instead feeds an arbitrary operation that started at timestamp 13, rather than a Put(77) that started before 12. You should find no valid possibilities left after exhaustive stepping, meaning a linearizability violation is detected: `n1`'s Get that finished at timestamp 12 cannot observe a value of 77.

</details>

## Optimizations

Potential performance/ergonomics optimizations to be implemented:

- [x] Special termination operation for finite execution in examples
- [ ] Multi-threaded stepping loop to boost checker performance
- [ ] Use shared per-node queues and only track indices/pointers in each possibility
