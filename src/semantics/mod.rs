/// Types for working with elements in the Domain of Discourse.
pub mod elements;

use elements::{ArgumentMap, Arguments, ElementQuantifier, ElementSet, Existential};

// pub mod assertions {
//     mod implies;

//     pub use implies::implies;
// }

/// Predefined functions
pub mod functions {
    // mod function_implementation;
    // pub use function_implementation::FunctionImplementation;
}

/// Predefined predicates
pub mod predicates {
    mod conjunction;
    // mod disjunction;
    // mod linked_predicate;
    mod negation;
    mod true_for_arguments;
    mod undetermined;
    mod universally_obeyed;

    pub use conjunction::Conjunction;
    // pub use disjunction::Disjunction;
    // pub use linked_predicate::LinkedPredicate;
    pub use negation::Negation;
    pub use true_for_arguments::TrueForArguments;
    pub use undetermined::Undetermined;
    pub use universally_obeyed::UniversallyObeyed;
}

mod graph;

pub use graph::{
    Function, FunctionNode, GraphNode, GraphTraversalSignature, Predicate, PredicateNode,
};
