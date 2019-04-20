extern crate serde_json;

use evalexpr::build_operator_tree;
use evalexpr::Context;
use evalexpr::EvalexprError::CustomMessage;
use evalexpr::HashMapContext;
use evalexpr::Value;
use evalexpr::*;
use pathfinding::prelude::topological_sort;
use std::collections::HashMap;

use std::io;

fn if_fn() -> Function {
    return Function::new(
        None,
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
                _ => Err(CustomMessage("if_fn".into())),
            }
        }),
    );
}

fn abs_fn() -> Function {
    return Function::new(
        None,
        Box::new(|arguments| {
            let number = &arguments[0];
            match number {
                Value::Float(f) => Ok(Value::from(f.abs())),
                Value::Int(i) => Ok(Value::from(i.abs())),
                _ => Err(EvalexprError::expected_number(number.clone())),
            }
        }),
    );
}

fn sum_fn() -> Function {
    return Function::new(
        None,
        Box::new(|arguments| {
            let mut total = 0.0;
            for argument in arguments {
                if let Value::Float(float) = argument {
                    total += *float;
                } else if let Value::Int(int) = argument {
                    total += *int as f64;
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }
            return Ok(Value::from(total));
        }),
    );
}

fn avg_fn() -> Function {
    return Function::new(
        None,
        Box::new(|arguments| {
            let mut total = 0.0;
            for argument in arguments {
                if let Value::Float(float) = argument {
                    total += *float;
                } else if let Value::Int(int) = argument {
                    total += *int as f64;
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }
            let avg = total / arguments.len() as f64;
            Ok(Value::from(avg))
        }),
    );
}

fn max_fn() -> Function {
    return Function::new(
        None,
        Box::new(|arguments| {
            let mut max_int = IntType::min_value();
            let mut max_float = -1.0f64 / 0.0f64;
            debug_assert!(max_float.is_infinite());

            for argument in arguments {
                if let Value::Float(float) = argument {
                    max_float = max_float.max(*float);
                } else if let Value::Int(int) = argument {
                    max_int = max_int.max(*int);
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }

            if (max_int as FloatType) > max_float {
                Ok(Value::Int(max_int))
            } else {
                Ok(Value::Float(max_float))
            }
        }),
    );
}

fn safe_div_fn() -> Function {
    return Function::new(
        None,
        Box::new(|arguments| {
            let a = &arguments[0];
            let b = &arguments[1];

            let mut a_as_f = 0.0;
            if let Value::Float(float) = a {
                a_as_f = *float;
            } else if let Value::Int(int) = a {
                a_as_f = *int as f64;
            } else {
                return Err(EvalexprError::expected_number(a.clone()));
            }

            match b {
                Value::Float(0.0) => Ok(Value::from(0.0)),
                Value::Int(0) => Ok(Value::from(0)),
                Value::Int(i) => Ok(Value::from(*i as f64 / a_as_f)),
                Value::Float(f) => Ok(Value::from(f / a_as_f)),
                _ => panic!("safe_div"),
            }
        }),
    );
}

fn create_context() -> Result<HashMapContext, String> {
    let mut context = HashMapContext::new();

    context.set_function("if".into(), if_fn()).unwrap();
    context.set_function("IF".into(), if_fn()).unwrap();

    context.set_function("abs".into(), abs_fn()).unwrap();
    context.set_function("ABS".into(), abs_fn()).unwrap();

    context.set_function("sum".into(), sum_fn()).unwrap();
    context.set_function("SUM".into(), sum_fn()).unwrap();

    context.set_function("avg".into(), avg_fn()).unwrap();
    context.set_function("AVG".into(), avg_fn()).unwrap();

    context.set_function("max".into(), max_fn()).unwrap(); // TODO implement max
    context.set_function("MAX".into(), max_fn()).unwrap(); // TODO implement max

    context
        .set_function("safe_div".into(), safe_div_fn())
        .unwrap();
    context
        .set_function("SAFE_DIV".into(), safe_div_fn())
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

    let mut solution: HashMap<String, serde_json::value::Value> = HashMap::new();

    for identitier in sorted.unwrap().iter().rev() {
        let equation = parsed_equations.get(&identitier);

        match equation.unwrap().eval_with_context(&context) {
            Ok(value) => {
                solution.insert(
                    identitier.clone(),
                    match value.clone() {
                        evalexpr::Value::Int(i) => serde_json::Value::from(i),
                        evalexpr::Value::String(s) => serde_json::Value::from(s),
                        evalexpr::Value::Float(f) => serde_json::Value::from(f),
                        evalexpr::Value::Boolean(b) => serde_json::Value::from(b),
                        evalexpr::Value::Tuple(_) => serde_json::Value::from(""),
                        evalexpr::Value::Empty => serde_json::Value::from(""),
                    },
                );
                context.set_value(identitier.clone(), value).unwrap();
            }
            Err(error) => {
                println!(
                    "ERRROR on variable {} : {} while evaluating {}",
                    identitier,
                    error,
                    problem.get(identitier).unwrap()
                );
                panic!(error);
            }
        }
    }
    let ouput = serde_json::to_string(&solution).unwrap();
    println!("{}", ouput);
}
