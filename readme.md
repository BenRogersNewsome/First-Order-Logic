# First Order Logic

A Rust implementation of First Order Logic.

Goals:
1. Construct and manipulate logical statements using the syntax of first-order logic.
    1. Convert logical statements into Prenex Normal Form (PNF), Skolem Normal Form (SNF) and Clause Normal Form (CNF).
2. Define, and make assertions on, predicates and functions.
    1. Ask logical questions about predicates and functions, to receive answers of True, False or Undetermined.
3. Expose an interface for creating extra predicates, assertions and functions, which can be used interoperably.

Non-goals:
- Implement any formalism of set theory, or expose the idea of a set.
- Implement second-order, or higher-order logics.

# Package Structure

## Syntax

Exposes an interface for dealing with first-order logic on a purely syntactic level, including methods for converting formulae into various normal forms.

## Graph

Provides the logic which backs the semantic engine of first-order logic, using an in-memory graph structure. The graph is rarely operated on directly but is managed by the public interface created by the many default predicates and functions. 

## Formula

Provides an interface for Clause-Normal formulae that can be operated on, and queried, in a semantic way.


# Strong vs Weak Typing for formula definitions

In general, strong static typing is more useful when we will need to query the formula directly - i.e for inspecting it in some external package, whereas weak typing (in other words, enum typing) is more useful when we wish to walk the formula and produce outputs in first-order logic.

For this reason, this package allows both situations. The `syntax` package defines an interface for strongly-typed syntactic expressions, which can be queried and manipulated without worrying about the underlying semantics, and the `formula` package defines a weakly typed CNF formula, which can be walked 