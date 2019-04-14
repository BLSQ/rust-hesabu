use evalexpr::build_operator_tree;
use evalexpr::Context;
use evalexpr::HashMapContext;
use evalexpr::Value;

use std::collections::HashMap;

fn main() {
    let problem = [
        ["c", "a + 10 * b"],
        ["a", "10"],
        ["b", "10+a"],
        ["d", "a + (b)"],
        //   ["err", "sin(a"],
    ];

    let mut parsed_equations = HashMap::new(); //<String,Node>()

    for key_value in &problem {
        let key = key_value[0];
        let equation = key_value[1];
        println!("PROCESSING : {} => {}", key, equation);
        let node = build_operator_tree(equation);
        match node {
            Ok(tree) => {
                parsed_equations.insert(key, tree);

                for identifier in parsed_equations.get(&key).unwrap().iter_identifiers() {
                    println!("{}", identifier)
                }
                // ...
            }
            Err(error) => println!("{}", error),
        }
    }

    let toposort = ["a", "b", "c", "d"];

    let mut context = HashMapContext::new();

    for identitier in &toposort {
        let equation = parsed_equations.get(identitier);

        match equation.unwrap().eval_with_context(&context) {
            Ok(value) => {
                println!("{}", &value);
                context.set_value(identitier.to_string(), value).unwrap();
            }
            Err(error) => {
                println!("{}", error);
                panic!(error);
            }
        }
    }
}
