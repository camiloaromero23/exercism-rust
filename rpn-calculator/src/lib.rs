#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }
    let mut rpn: Vec<i32> = Vec::new();
    for i in inputs {
        match i {
            CalculatorInput::Value(number) => rpn.push(*number),
            _ => {
                if rpn.len() < 2 {
                    return None;
                }
                let (num1, num2) = (rpn.pop().unwrap(), rpn.pop().unwrap());

                match i {
                    CalculatorInput::Add => rpn.push(num1 + num2),
                    CalculatorInput::Multiply => rpn.push(num1 * num2),
                    CalculatorInput::Subtract => rpn.push(num2 - num1),
                    CalculatorInput::Divide => rpn.push(num2 / num1),
                    _ => return None,
                }
            }
        }
    }
    if rpn.len() != 1 {
        return None;
    }

    rpn.pop()
}
