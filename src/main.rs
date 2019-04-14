use evalexpr::build_operator_tree;
use evalexpr::Context;
use evalexpr::HashMapContext;
use evalexpr::Value;
use evalexpr::*;

use std::collections::HashMap;

fn main() {
    let problem = [
        ["c", "a + 10 * b"],
        ["a", "10"],
        ["b", "if(a<=10.0, 2, 10+a)"],
        ["d", "a + (b)"],
        //   ["err", "sin(a"],
    ];

    let mut parsed_equations = HashMap::new(); //<String,Node>()

    for key_value in &problem {
        let key = key_value[0];
        let equation = key_value[1];
        let node = build_operator_tree(equation);
        match node {
            Ok(tree) => {
                parsed_equations.insert(key, tree);

                for identifier in parsed_equations.get(&key).unwrap().iter_identifiers() {
                    //println!("{}", identifier)
                }
                // ...
            }
            Err(error) => println!("{}", error),
        }
    }

    let toposort = ["a", "b", "c", "d"];

    let mut context = HashMapContext::new();

    context
        .set_function(
            "if".into(),
            Function::new(
                Some(3), /* argument amount */
                Box::new(|arguments| {
                    let condition = &arguments[0];

                    match condition {
                        Value::Boolean(bool) => {
                            if *bool {
                                println!("condition is {}", true);
                                Ok(arguments[1].clone())
                            } else {
                                println!("condition is {}", false);
                                Ok(arguments[2].clone())
                            }
                        }
                        _ => panic!("unknown"),
                    }
                }),
            ),
        )
        .unwrap();

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
