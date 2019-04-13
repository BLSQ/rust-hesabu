use evalexpr::build_operator_tree;

use std::collections::HashMap;

fn main() {
    let mut problem = [["c", "a + 10 * b"], ["a", "10"], ["b", "10+a"]];

    for key_value in &problem {
        let key = key_value[0];
        let equation = key_value[1];
        println!("PROCESSING : {} => {}", key, equation);

        if let Ok(tree) = build_operator_tree(equation) {
            for identifier in tree.iter_identifiers() {
                println!("{}", identifier)
            }
            // ...
        }
    }
}
