# Retrieving information about the members of a given group

Information in the shape of the a `KeyPackage` can either be retrieved in bulk for all group members or for individual ones.

## Retrieving a list of group members

A list of the `KeyPackage`s of current group members can be retrieved by using the `.members()` function.

```rust,no_run,noplayground
{{#include ../../../openmls/tests/book_code.rs:retrieve_members}}
```

## Retrieving individual group members

The `KeyPackage` of an individual group member can be retrieved by using the `.member()` function using the member's `KeyPackageRef` as input. `KeyPackageRef`s are generally used to identify members within a group. For example, the `.sender()` function of `UnverifiedMessage` yields a `Sender` enum, which for the `Member` element yields the sender's `KeyPackageRef`.

```rust,no_run,noplayground
{{#include ../../../openmls/tests/book_code.rs:member_lookup}}
```
