use evalexpr::build_operator_tree;
use evalexpr::Context;
use evalexpr::HashMapContext;
use evalexpr::Value;
use evalexpr::*;
use pathfinding::prelude::topological_sort;
use std::collections::HashMap;

fn main() {
    let problem = [
        ["c", "a + 10 * b"],
        ["a", "10"],
        ["b", "if(a<=10.0, 2, 10+a)"],
        ["d", "a + b"],
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
                println!("processing {} := {}", key, equation);

                for identifier in parsed_equations
                    .get(&key)
                    .unwrap()
                    .iter_variable_identifiers()
                {
                    println!("{}", identifier);
                }
            }
            Err(error) => println!("{}", error),
        }
    }
    let variable_identifiers = |key:String| -> Vec<String> {
        parsed_equations
                    .get(&key.as_ref())
                    .unwrap()
                    .iter_variable_identifiers().map(|x| x.to_string()).collect()
    };
    fn successors(node: &String) -> Vec<String> {
        match node.as_ref() {
            "a" => vec![],
            "b" => vec!["a".to_string(),"a".to_string()],
            "c" => vec!["a".to_string(),"b".to_string()],
            "d" => vec!["a".to_string(),"b".to_string()],
            _ => vec![]
        }
    }
    let mut keys: Vec<String> = Vec::new();
    for (k, v) in parsed_equations.iter() {
        keys.push(k.to_string());
    }

    // TODO how to use variable_identifiers as successors ?
    let sorted = topological_sort(&keys, successors);

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

    for identitier in sorted.unwrap().iter().rev() {
        let equation = parsed_equations.get(&identitier.as_ref());

        match equation.unwrap().eval_with_context(&context) {
            Ok(value) => {
                println!("solution for {} {}", identitier, &value);
                context.set_value(identitier.clone(), value).unwrap();
            }
            Err(error) => {
                println!("{}", error);
                panic!(error);
            }
        }
    }
}
