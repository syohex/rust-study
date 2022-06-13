use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Op {
    Exit,
    Error,
    Push,
    GetLocal,
    SetLocal,
    Equal,
    LessThan,
    IfTrue,
    IfNot,
    Add,
    Sub,
    ReadInt,
    Print,
}

#[derive(Clone, Debug)]
enum Value {
    None,
    Index(usize),
    IntValue(i64),
    //    String(String),
}

impl Value {
    fn is_none(&self) -> bool {
        match self {
            Value::None => true,
            _ => false,
        }
    }

    fn unwrap_index(&self) -> usize {
        match self {
            Value::Index(idx) => *idx,
            _ => panic!("value is not an index: {:?}", self),
        }
    }

    fn unwrap_int(&self) -> i64 {
        match self {
            Value::IntValue(val) => *val,
            _ => panic!("value is not an integer value: {:?}", self),
        }
    }
}

struct Insn {
    op: Op,
    immediate: Value,
}

impl fmt::Debug for Insn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Insn {{ op: {:10} imm: {:?} }}",
            format!("{:?}", self.op),
            self.immediate
        )
    }
}

#[derive(Debug)]
struct Program {
    insns: Vec<Insn>,
    local_indexes: HashMap<String, usize>,
}

impl Program {
    fn new() -> Self {
        Self {
            insns: vec![],
            local_indexes: HashMap::new(),
        }
    }

    fn append_insn(&mut self, op: Op) {
        self.insns.push(Insn {
            op: op,
            immediate: Value::None,
        });
    }

    fn append_insn_immediate(&mut self, op: Op, imm: Value) {
        self.insns.push(Insn {
            op: op,
            immediate: imm,
        });
    }

    fn find_local(&self, identifier: &str) -> Option<usize> {
        match self.local_indexes.get(identifier) {
            Some(index) => Some(*index),
            None => None,
        }
    }

    fn declare_local(&mut self, identifier: &str) -> usize {
        assert!(self.find_local(identifier).is_none());

        let local_index = self.local_indexes.len();
        self.local_indexes
            .insert(identifier.to_owned(), local_index);
        local_index
    }
}

struct Input {
    chars: Vec<char>,
    pos: usize,
}

impl Input {
    fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek_char(&self) -> char {
        if self.pos >= self.chars.len() {
            return '\0';
        }

        self.chars[self.pos]
    }

    fn eat_char(&mut self) -> char {
        let ch = self.peek_char();
        if ch != '\0' {
            self.pos += 1;
        }

        ch
    }

    fn eat_whitespaces(&mut self) {
        loop {
            let ch = self.peek_char();
            match ch {
                ' ' | '\t' | '\r' | '\n' => {}
                _ => return,
            }

            self.eat_char();
        }
    }

    fn eat_comment(&mut self) {
        loop {
            let ch = self.peek_char();

            match ch {
                '\n' => {
                    self.eat_char();
                    break;
                }
                '\0' => {
                    break;
                }
                _ => {
                    self.eat_char();
                }
            }
        }
    }

    fn match_token(&mut self, token: &str) -> bool {
        self.eat_whitespaces();

        let token_chars: Vec<char> = token.chars().collect();
        let num_chars = token_chars.len();

        if self.pos + num_chars > self.chars.len() {
            return false;
        }

        if self.chars[self.pos..(self.pos + num_chars)] == token_chars {
            self.pos += num_chars;
            self.eat_whitespaces();
            return true;
        }

        false
    }

    fn expect_token(&mut self, token: &str) {
        if !self.match_token(token) {
            panic!("expected token '{token}'");
        }
    }

    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();

        loop {
            let ch = self.peek_char();
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            identifier.push(ch);
            self.eat_char();
        }

        if identifier.is_empty() {
            panic!("expected identifier");
        }

        identifier
    }

    fn parse_integer(&mut self) -> i64 {
        let mut num = 0i64;

        loop {
            let ch = self.peek_char();
            if !ch.is_digit(10) {
                break;
            }

            let digit = (ch as i64) - ('0' as i64);
            num = 10 * num + digit;

            self.eat_char();
        }

        num
    }
}

fn parse_atom(input: &mut Input, prog: &mut Program) {
    if input.match_token("(") {
        parse_expr(input, prog);
        input.expect_token(")");
        return;
    }

    if input.match_token("read_int") {
        prog.append_insn(Op::ReadInt);
        return;
    }

    let ch = input.peek_char();
    if ch.is_digit(10) {
        let num = input.parse_integer();
        prog.append_insn_immediate(Op::Push, Value::IntValue(num));
        return;
    }

    if ch.is_alphanumeric() || ch == '_' {
        let identifier = input.parse_identifier();
        let local_index = prog.find_local(&identifier);
        if local_index.is_none() {
            panic!("referent to undeclared variable '{identifier}'");
        }

        prog.append_insn_immediate(Op::GetLocal, Value::Index(local_index.unwrap()));
        return;
    }

    panic!("invalid atomic expression");
}

fn parse_expr(input: &mut Input, prog: &mut Program) {
    parse_atom(input, prog);

    if input.match_token("+") {
        parse_atom(input, prog);
        prog.append_insn(Op::Add);
    } else if input.match_token("-") {
        parse_atom(input, prog);
        prog.append_insn(Op::Sub);
    } else if input.match_token("==") {
        parse_atom(input, prog);
        prog.append_insn(Op::Equal);
    } else if input.match_token("<") {
        parse_atom(input, prog);
        prog.append_insn(Op::LessThan);
    }
}

fn parse_statement(input: &mut Input, prog: &mut Program) {
    input.eat_whitespaces();

    if input.match_token("#") {
        input.eat_comment();
    } else if input.match_token("let") {
        let identifier = input.parse_identifier();

        input.expect_token("=");

        parse_expr(input, prog);

        if let Some(_) = prog.find_local(&identifier) {
            panic!("local variable {identifier} is already declared");
        }

        let local_index = prog.declare_local(&identifier);
        prog.append_insn_immediate(Op::SetLocal, Value::Index(local_index));
    } else if input.match_token("if") {
        parse_expr(input, prog);

        input.expect_token("then");

        let ifnot_insn_index = prog.insns.len();
        prog.append_insn(Op::IfNot);

        parse_statement(input, prog);

        let jump_to_index = prog.insns.len();
        let jump_offset = (jump_to_index as i64) - (ifnot_insn_index as i64) - 1;
        prog.insns[ifnot_insn_index].immediate = Value::IntValue(jump_offset);
    } else if input.match_token("begin") {
        loop {
            if input.match_token("end") {
                break;
            }

            parse_statement(input, prog);
        }
    } else if input.match_token("print") {
        parse_expr(input, prog);
        prog.append_insn(Op::Print);
    } else if input.match_token("assert") {
        parse_expr(input, prog);

        prog.append_insn_immediate(Op::IfTrue, Value::IntValue(1));
        prog.append_insn(Op::Error);
    } else if input.match_token("exit") {
        prog.append_insn(Op::Exit);
    } else {
        panic!("invalid statement");
    }
}

fn parse_file(file_name: &str) -> Program {
    let input_str = fs::read_to_string(file_name).expect("could not read input file");
    let mut input = Input::new(input_str);
    let mut program = Program::new();

    loop {
        if input.peek_char() == '\0' {
            break;
        }

        parse_statement(&mut input, &mut program);
    }

    program
}

struct VM {
    locals: Vec<Value>,
    stack: Vec<Value>,
    pc: usize,
}

impl VM {
    fn new() -> Self {
        Self {
            locals: vec![],
            stack: vec![],
            pc: 0,
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn eval(&mut self, prog: Program) {
        let num_locals = prog.local_indexes.len();
        self.locals.resize(num_locals, Value::None);

        self.pc = 0;

        while self.pc < prog.insns.len() {
            let insn = &prog.insns[self.pc];

            match insn.op {
                Op::Exit => {
                    return;
                }
                Op::Error => {
                    panic!("Run time error!!");
                }
                Op::Push => {
                    self.push(insn.immediate.clone());
                }
                Op::GetLocal => {
                    let val = self.locals[insn.immediate.unwrap_index()].clone();
                    assert!(!val.is_none(), "uninitialized local variable");
                    self.push(val);
                }
                Op::SetLocal => {
                    self.locals[insn.immediate.unwrap_index()] = self.pop();
                }
                Op::Equal => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();

                    let val = if arg0 == arg1 { 1 } else { 0 };
                    self.push(Value::IntValue(val));
                }
                Op::LessThan => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();

                    let val = if arg0 < arg1 { 1 } else { 0 };
                    self.push(Value::IntValue(val));
                }
                Op::IfTrue => {
                    let test_val = self.pop().unwrap_int();
                    if test_val != 0 {
                        let jump_offset = insn.immediate.unwrap_int();
                        self.pc = ((self.pc as i64) + jump_offset) as usize;
                    }
                }
                Op::IfNot => {
                    let test_val = self.pop().unwrap_int();
                    if test_val == 0 {
                        let jump_offset = insn.immediate.unwrap_int();
                        self.pc = ((self.pc as i64) + jump_offset) as usize;
                    }
                }
                Op::Add => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    self.push(Value::IntValue(arg0 + arg1));
                }

                Op::Sub => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    self.push(Value::IntValue(arg0 - arg1));
                }
                Op::ReadInt => {
                    println!("Input an integer value:");
                    print!("> ");

                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let n: i64 = input.trim().parse().unwrap();
                    self.push(Value::IntValue(n));
                }
                Op::Print => {
                    let val = self.pop().unwrap_int();
                    println!("print: {val}");
                }
            }

            self.pc += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file.bas>", args[0]);
        return;
    }

    let prog = parse_file(&args[1]);
    let mut vm = VM::new();
    vm.eval(prog);
}
