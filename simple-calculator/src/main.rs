#[derive(Debug, PartialEq, Eq)]
enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Op(Operator),
    Num(i64),
}

fn to_operator(ch: char) -> Operator {
    match ch {
        '+' => Operator::ADD,
        '-' => Operator::SUB,
        '*' => Operator::MUL,
        '/' => Operator::DIV,
        _ => panic!("invalid operator"),
    }
}

fn str_to_rpn(input: &str) -> Result<Vec<Operand>, &str> {
    let cs: Vec<char> = input.chars().collect();
    let limit = cs.len();
    let mut pos = 0usize;

    let mut ret = vec![];
    let mut ops = vec![];
    while pos < limit {
        while pos < limit && cs[pos].is_whitespace() {
            pos += 1;
        }

        if cs[pos].is_ascii_digit() {
            let mut sum = 0;
            while pos < limit && cs[pos].is_ascii_digit() {
                if let Some(n) = cs[pos].to_digit(10) {
                    sum = 10 * sum + n as i64;
                } else {
                    return Err("invalid character");
                }

                pos += 1;
            }

            ret.push(Operand::Num(sum));
        } else if cs[pos] == '+' || cs[pos] == '-' {
            if ops.is_empty() {
                ops.push(cs[pos]);
            } else {
                while !ops.is_empty() && (ops[ops.len() - 1] == '+' || ops[ops.len() - 1] == '-') {
                    if let Some(c) = ops.pop() {
                        ret.push(Operand::Op(to_operator(c)));
                    }
                }
                ops.push(cs[pos]);
            }

            pos += 1;
        } else if cs[pos] == '*' || cs[pos] == '/' {
            if ops.is_empty() {
                ops.push(cs[pos]);
            } else {
                while !ops.is_empty() && (ops[ops.len() - 1] == '*' || ops[ops.len() - 1] == '/') {
                    if let Some(c) = ops.pop() {
                        ret.push(Operand::Op(to_operator(c)));
                    }
                }
                ops.push(cs[pos]);
            }

            pos += 1;
        } else if cs[pos] == '(' {
            ops.push(cs[pos]);
            pos += 1;
        } else if cs[pos] == ')' {
            if ops.is_empty() {
                return Err("not found open parenthesis");
            }

            let mut ok = false;
            while !ops.is_empty() {
                if let Some(c) = ops.pop() {
                    if c == '(' {
                        ok = true;
                        break;
                    }

                    ret.push(Operand::Op(to_operator(c)));
                }
            }

            if !ok {
                return Err("not found open parenthesis");
            }

            pos += 1;
        }
    }

    while let Some(op) = ops.pop() {
        if op == '(' {
            return Err("missing close parenthesis");
        }
        ret.push(Operand::Op(to_operator(op)));
    }

    Ok(ret)
}

fn calculate(operands: &[Operand]) -> Option<i64> {
    let mut stack = vec![];
    for operand in operands {
        match *operand {
            Operand::Num(n) => stack.push(n),
            Operand::Op(Operator::ADD) => {
                if stack.len() < 2 {
                    return None;
                }

                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 + op2);
            }
            Operand::Op(Operator::SUB) => {
                if stack.len() < 2 {
                    return None;
                }

                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 - op2);
            }
            Operand::Op(Operator::MUL) => {
                if stack.len() < 2 {
                    return None;
                }

                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 * op2);
            }
            Operand::Op(Operator::DIV) => {
                if stack.len() < 2 {
                    return None;
                }

                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 / op2);
            }
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        dbg!(stack);
        None
    }
}

fn main() {
    let ret = str_to_rpn("1+2-3");
    println!("ret={:?}", ret);

    let value = calculate(&ret.unwrap());
    println!("1+2-3={}", value.unwrap());
}

#[test]
fn test_str_to_rpn() {
    {
        let input = "1 + 1";
        let ret = str_to_rpn(input);
        assert!(ret.is_ok());

        let ret = ret.unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(
            ret,
            vec![Operand::Num(1), Operand::Num(1), Operand::Op(Operator::ADD)]
        );

        let value = calculate(&ret);
        assert_eq!(value, Some(2));
    }
    {
        let inputs = vec!["1 - 5)", "(1 - 5"];
        for input in inputs {
            let ret = str_to_rpn(input);
            assert!(ret.is_err());
        }
    }
    {
        let input = "32 + 4 * 2 / (1 - 5)";
        let ret = str_to_rpn(input);
        assert!(ret.is_ok());

        let ret = ret.unwrap();
        assert_eq!(ret.len(), 9);
        assert_eq!(
            ret,
            vec![
                Operand::Num(32),
                Operand::Num(4),
                Operand::Num(2),
                Operand::Op(Operator::MUL),
                Operand::Num(1),
                Operand::Num(5),
                Operand::Op(Operator::SUB),
                Operand::Op(Operator::DIV),
                Operand::Op(Operator::ADD),
            ],
        );

        let value = calculate(&ret);
        assert_eq!(value, Some(30));
    }
}
