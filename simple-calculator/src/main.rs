#[derive(Debug)]
enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[derive(Debug)]
enum Operand {
    Op(Operator),
    Num(i64)
}

fn str_to_rpn(input: &str) -> Result<Vec<Operand>, &str> {
    let mut ret = vec![];
    for c in input.chars() {
        match c {
            '0'..='9' => {
                let n = c as i64 - '0' as i64;
                ret.push(Operand::Num(n));
            }
            '+'  => ret.push(Operand::Op(Operator::ADD)),
            '-'  => ret.push(Operand::Op(Operator::SUB)),
            _ => return Err("invalid input"),
        }
    }

    Ok(ret)
}

fn main() {
    let ret = str_to_rpn("1+2-3");
    println!("ret={:?}", ret);
}
