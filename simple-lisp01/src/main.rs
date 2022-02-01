use std::collections::HashMap;
use std::fmt;
use std::io;
use std::num::ParseFloatError;

#[derive(Debug)]
enum LispError {
    Reason(String),
}

#[derive(Clone)]
enum LispExpression {
    Symbol(String),
    Number(f64),
    List(Vec<LispExpression>),
    Func(fn(&[LispExpression]) -> Result<LispExpression, LispError>),
}

impl fmt::Display for LispExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LispExpression::Symbol(s) => s.clone(),
            LispExpression::Number(n) => n.to_string(),
            LispExpression::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            LispExpression::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{s}")
    }
}

#[derive(Clone)]
struct LispEnv {
    data: HashMap<String, LispExpression>,
}

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(token: &'a [String]) -> Result<(LispExpression, &'a [String]), LispError> {
    let (token, rest) = token
        .split_first()
        .ok_or(LispError::Reason("could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(LispError::Reason("unexpected ')'".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(LispExpression, &'a [String]), LispError> {
    let mut res: Vec<LispExpression> = vec![];
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(LispError::Reason("could not find closing ')'".to_string()))?;
        if next_token == ")" {
            return Ok((LispExpression::List(res), rest));
        }

        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> LispExpression {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => LispExpression::Number(v),
        Err(_) => LispExpression::Symbol(token.to_string().clone()),
    }
}

fn default_env() -> LispEnv {
    let mut data: HashMap<String, LispExpression> = HashMap::new();
    data.insert(
        "+".to_string(),
        LispExpression::Func(
            |args: &[LispExpression]| -> Result<LispExpression, LispError> {
                let sum = parse_list_of_floats(args)?
                    .iter()
                    .fold(0.0, |sum, a| sum + a);
                Ok(LispExpression::Number(sum))
            },
        ),
    );
    data.insert(
        "-".to_string(),
        LispExpression::Func(
            |args: &[LispExpression]| -> Result<LispExpression, LispError> {
                let floats = parse_list_of_floats(args)?;
                let first = *floats.first().ok_or(LispError::Reason(
                    "expected at least one number".to_string(),
                ))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
                Ok(LispExpression::Number(first - sum_of_rest))
            },
        ),
    );

    LispEnv { data }
}

fn parse_list_of_floats(args: &[LispExpression]) -> Result<Vec<f64>, LispError> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn parse_single_float(exp: &LispExpression) -> Result<f64, LispError> {
    match exp {
        LispExpression::Number(num) => Ok(*num),
        _ => Err(LispError::Reason("expected a number".to_string())),
    }
}

fn eval(exp: &LispExpression, env: &mut LispEnv) -> Result<LispExpression, LispError> {
    match exp {
        LispExpression::Symbol(k) => env
            .data
            .get(k)
            .ok_or(LispError::Reason(format!("unexpected symbol k='{k}'")))
            .map(|x| x.clone()),
        LispExpression::Number(_) => Ok(exp.clone()),
        LispExpression::List(list) => {
            let first_form = list
                .first()
                .ok_or(LispError::Reason("expected a non empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                LispExpression::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<LispExpression>, LispError>>();
                    f(&args_eval?)
                }
                _ => Err(LispError::Reason(
                    "first form must be a function".to_string(),
                )),
            }
        }
        LispExpression::Func(_) => Err(LispError::Reason("unexpected form".to_string())),
    }
}

fn parse_eval(expr: String, env: &mut LispEnv) -> Result<LispExpression, LispError> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line from stdin");
    expr
}

fn main() {
    let env = &mut default_env();
    loop {
        print!("> ");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("// => {res}"),
            Err(e) => match e {
                LispError::Reason(msg) => println!("// err => {msg}"),
            },
        }
    }
}
