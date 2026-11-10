//! What a Stack is: its kind and state, the owner it runs on, its compose
//! definition and the rules it must follow, what a redeploy would change, and
//! telling Homeport's Stacks from the ones started outside it.

pub mod definition;
pub mod diff;
pub mod external;
pub(crate) mod listing;
pub mod owner;
pub mod stack;
