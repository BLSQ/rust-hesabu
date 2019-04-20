#[macro_use]
extern crate serde_json;

use evalexpr::build_operator_tree;
use evalexpr::Context;
use evalexpr::HashMapContext;
use evalexpr::Value;
use evalexpr::*;
use pathfinding::prelude::topological_sort;
use std::collections::HashMap;

use std::io;

fn create_context() -> Result<HashMapContext, String> {
    let mut context = HashMapContext::new();

    context
        .set_function(
            "if".into(),
            Function::new(
                Some(3),
                Box::new(|arguments| {
                    let condition = &arguments[0];

                    match condition {
                        Value::Boolean(bool) => {
                            if *bool {
                                Ok(arguments[1].clone())
                            } else {
                                Ok(arguments[2].clone())
                            }
                        }
                        _ => panic!("unknown"),
                    }
                }),
            ),
        )
        .unwrap();
    return Ok(context);
}

fn main() {
    let problem: HashMap<String, String> = serde_json::from_reader(io::stdin()).unwrap();

    let mut parsed_equations = HashMap::new(); //<String,Node>()

    for (key, equation) in &problem {
        let node = build_operator_tree(equation);
        match node {
            Ok(tree) => {
                parsed_equations.insert(key, tree);
            }
            Err(error) => panic!("ERROR : {} while parsing {} {}", error, key, equation),
        }
    }
    let variable_identifiers = |key: &String| -> Vec<String> {
        parsed_equations
            .get(&key)
            .unwrap()
            .iter_variable_identifiers()
            .map(|x| x.to_string())
            .collect()
    };

    let keys: Vec<String> = parsed_equations.keys().map(|k| k.to_string()).collect();

    let sorted = topological_sort(&keys, variable_identifiers);

    let mut context = create_context().unwrap();

    let mut solution = HashMap::new();

    for identitier in sorted.unwrap().iter().rev() {
        let equation = parsed_equations.get(&identitier);

        match equation.unwrap().eval_with_context(&context) {
            Ok(value) => {
                solution.insert(identitier.clone(), value.clone());
                context.set_value(identitier.clone(), value).unwrap();
            }
            Err(error) => {
                println!("ERRROR {}", error);
                panic!(error);
            }
        }
    }
    let ouput = serde_json::to_string(&solution).unwrap();
    println!("{}", ouput)
}
