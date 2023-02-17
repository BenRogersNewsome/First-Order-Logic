# Rust First Order Logic

A Rust implementation of the syntax of First Order Logic, and a logical 'harness' for making self-consistent logical assertions on a domain of discourse.

**Goals**:
- Construct and manipulate logical statements using the syntax of first-order logic.
- Define, and make assertions on, predicates and functions with self-consistent logical checking
- Expose an interface for integrating packages that implement (independent of this package) higher-level mathematical concepts (such as set theory) and interfacing with them through the language of FOL.

**Non-goals**:
- Implement any formalism of set theory, or expose the idea of a set.
- Automated theorem proving
- Provide a base on which higher-level mathematical packages can be implemented.

## Features

| Feature                      | Description                                                         | Status |
| ---------------------------- | ------------------------------------------------------------------- | :----: |
| **Logical Syntactics**       |                                                                     |        |
| FOL grammar                  | A typed grammar for FOL                                             |   ✅   |
| Prenex Normal Form           | A typing for PNF and conversion from others forms                   |   ✅   |
| Skolem Normal Form           | A typing for SNF and and conversion from other forms                |   ✅   |
| Conjunctive Normal Form      | A typing for CNF and conversion from other forms                    |   ✅   |
| **Logical Semantics**        |                                                                     |        |
| Predicates                   | Graph support for asserting logical predicates                      |  WIP   |
| Functions                    | Graph support for defining logical functions                        |  WIP   |
| Bound Variables              | Graph support for creating named bound variables                    |  WIP   |
| Logical sentence integration | The ability to directly apply syntactic statements to the FOL graph | Future |

## Package Structure
---
### Syntax

Defines a strongly-typed grammar for first-order logic, together with a set of strongly-typed normal forms, and methods for converting between them.

### Semantics

Defines an in-memory graph structure that keeps track of logical statements and allows for self-consistent logical checking.

The graph is rarely operated on directly but is managed by the public interface created by the many default predicates and functions.