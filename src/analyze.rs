use std::collections::HashMap;

use crate::ast::*;

// To do. need to check multiple stacks and whether the order is accurate
// Currently, check the duplication, and the existence in the process of parsing expression
pub fn check_stack(stack_vec: &Vec<Vec<StackParam>>) -> HashMap<String, u32> {
    // Key is identifier and value is count(for later usage of OP_DUP)
    let mut global_stack_table: HashMap<String, u32> = HashMap::new();

    for stack in stack_vec.iter() {
        let mut stack_table: HashMap<String, u32> = HashMap::new();
        // Duplication check is done for each stack
        for e in stack.iter().rev() {
            // check whether the name of argument already exists.
            if stack_table.get(&e.identifier.0).is_some() {
                panic!(
                    "The name of argument cannot be duplicate: {:?} already exists.",
                    e.identifier.0
                )
            };
            stack_table.insert(e.identifier.0.to_owned(), 1);
        }
        for (k, v) in stack_table {
            global_stack_table.insert(k, v);
        }
    }

    global_stack_table
}

pub fn analyze(ast: Vec<Statement>, input: Vec<Vec<StackParam>>, target: &Target) {}
