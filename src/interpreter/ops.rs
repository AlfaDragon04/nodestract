use crate::engine::value::Value;
use super::Interpreter;

impl Interpreter {
    pub fn eval_binary_op(&mut self, left: Value, operator: &str, right: Value) -> Value {
        match (left, right) {
            (Value::Null, Value::Null) => match operator {
                "==" => Value::Boolean(true),
                "!=" => Value::Boolean(false),
                _ => Value::Null,
            },
            (Value::Integer(a), Value::Integer(b)) => match operator {
                "+" => {
                    if let Some(res) = a.checked_add(b) {
                        Value::Integer(res)
                    } else {
                        let err_msg = "MATH ERROR: Integer overflow during addition.".to_string();
                        println!("{}", err_msg);
                        self.exception = Some(Value::String(err_msg));
                        Value::Null
                    }
                }
                "-" => {
                    if let Some(res) = a.checked_sub(b) {
                        Value::Integer(res)
                    } else {
                        let err_msg = "MATH ERROR: Integer underflow during subtraction.".to_string();
                        println!("{}", err_msg);
                        self.exception = Some(Value::String(err_msg));
                        Value::Null
                    }
                }
                "*" => {
                    if let Some(res) = a.checked_mul(b) {
                        Value::Integer(res)
                    } else {
                        let err_msg = "MATH ERROR: Integer overflow during multiplication.".to_string();
                        println!("{}", err_msg);
                        self.exception = Some(Value::String(err_msg));
                        Value::Null
                    }
                }
                "/" => {
                    if b == 0 {
                        let err_msg = "MATH ERROR: Division by zero.".to_string();
                        println!("{}", err_msg);
                        self.exception = Some(Value::String(err_msg));
                        Value::Null
                    } else {
                        Value::Integer(a / b)
                    }
                }
                ">" => Value::Boolean(a > b),
                "<" => Value::Boolean(a < b),
                ">=" => Value::Boolean(a >= b),
                "<=" => Value::Boolean(a <= b),
                "==" => Value::Boolean(a == b),
                "!=" => Value::Boolean(a != b),
                "&&" => Value::Boolean(a != 0 && b != 0),
                "||" => Value::Boolean(a != 0 || b != 0),
                _ => Value::Null,
            },
            (Value::Float(a), Value::Float(b)) => match operator {
                "+" => Value::Float(a + b),
                "-" => Value::Float(a - b),
                "*" => Value::Float(a * b),
                "/" => {
                    if b == 0.0 {
                        let err_msg = "MATH ERROR: Division by zero.".to_string();
                        println!("{}", err_msg);
                        self.exception = Some(Value::String(err_msg));
                        Value::Null
                    } else {
                        Value::Float(a / b)
                    }
                }
                ">" => Value::Boolean(a > b),
                "<" => Value::Boolean(a < b),
                ">=" => Value::Boolean(a >= b),
                "<=" => Value::Boolean(a <= b),
                "==" => Value::Boolean(a == b),
                "!=" => Value::Boolean(a != b),
                _ => Value::Null,
            },
            (Value::Boolean(a), Value::Boolean(b)) => match operator {
                "&&" => Value::Boolean(a && b),
                "||" => Value::Boolean(a || b),
                "==" => Value::Boolean(a == b),
                "!=" => Value::Boolean(a != b),
                _ => {
                    let err_msg = "TYPE ERROR: Invalid bool op".to_string();
                    println!("{}", err_msg);
                    self.exception = Some(Value::String(err_msg));
                    Value::Null
                }
            },
            (Value::Integer(a), Value::Float(b)) => {
                self.eval_binary_op(Value::Float(a as f64), operator, Value::Float(b))
            }
            (Value::Float(a), Value::Integer(b)) => {
                self.eval_binary_op(Value::Float(a), operator, Value::Float(b as f64))
            }
            (Value::String(a), Value::String(b)) => match operator {
                "+" => Value::String(a + &b),
                "==" => Value::Boolean(a == b),
                "!=" => Value::Boolean(a != b),
                _ => {
                    let err_msg = "TYPE ERROR: Invalid string op".to_string();
                    println!("{}", err_msg);
                    self.exception = Some(Value::String(err_msg));
                    Value::Null
                }
            },
            (Value::String(a), b) => match operator {
                "+" => Value::String(format!("{}{}", a, b)),
                "==" => Value::Boolean(false),
                "!=" => Value::Boolean(true),
                _ => {
                    let err_msg = format!("CRITICAL TYPE ERROR: Incompatible types for '{}': String and {:?}", operator, b);
                    println!("{}", err_msg);
                    self.exception = Some(Value::String(err_msg));
                    Value::Null
                }
            },
            (a, Value::String(b)) => match operator {
                "+" => Value::String(format!("{}{}", a, b)),
                "==" => Value::Boolean(false),
                "!=" => Value::Boolean(true),
                _ => {
                    let err_msg = format!("CRITICAL TYPE ERROR: Incompatible types for '{}': {:?} and String", operator, a);
                    println!("{}", err_msg);
                    self.exception = Some(Value::String(err_msg));
                    Value::Null
                }
            },
            (l, r) => match operator {
                "==" => Value::Boolean(false),
                "!=" => Value::Boolean(true),
                _ => {
                    let err_msg = format!(
                        "CRITICAL TYPE ERROR: Incompatible types for '{}': {:?} and {:?}",
                        operator, l, r
                    );
                    println!("{}", err_msg);
                    self.exception = Some(Value::String(err_msg));
                    Value::Null
                }
            },
        }
    }
}