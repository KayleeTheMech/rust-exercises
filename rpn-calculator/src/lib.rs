use std::vec;

#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn pop_operands_of_stack(stack: &mut Vec<CalculatorInput>) -> Option<(i32, i32)> {
    let operands = (stack.pop(), stack.pop());
    match operands {
        (Some(CalculatorInput::Value(op1)), Some(CalculatorInput::Value(op2))) => Some((op1, op2)),
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Value(_) => {
                stack.append(&mut vec![*input]);
            }
            _ => {
                let res: i32;
                match pop_operands_of_stack(&mut stack) {
                    Some((op1, op2)) => match input {
                        CalculatorInput::Add => res = op2 + op1,
                        CalculatorInput::Subtract => res = op2 - op1,
                        CalculatorInput::Multiply => res = op2 * op1,
                        CalculatorInput::Divide => res = op2 / op1,
                        _ => return None,
                    },
                    None => return None,
                }
                stack.append(&mut vec![CalculatorInput::Value(res)])
            }
        }
    }
    if stack.len() == 1 {
        match stack.pop() {
            Some(CalculatorInput::Value(result)) => return Some(result),
            _ => return None,
        }
    }
    return None;
}
