use crate::comp::Comp;
use crate::preset;
use crate::preset::BasicFn;
use std::collections::HashMap;

pub fn repl() {

    let mut variables: HashMap<[char; 5], Bat> = HashMap::new();
    let mut functions: HashMap<[char; 5], (u16, Vec<Bat>)> = HashMap::new();

    for var in preset::PRE_VAR {
        variables.insert(var.0, Bat::Val(var.1));
    }

    let mut ans: Comp;

    loop {
        let chain = split_input(take_input());

        match chain[0].as_str() {
            "var" => {
                ans = complete(tokenize(chain[2..].to_vec(), &variables, &functions), &functions);
                variables.insert( ['a', 'n', 's', ' ', ' '], Bat::Val(ans) );
                variables.insert( get_five(chain[1].as_str().to_string()), Bat::Val(ans) );
            },
            "def" => {
                functions.insert( get_five(chain[1].as_str().to_string()),
                ( chain[2].parse::<u16>().unwrap(), tokenize(chain[3..].to_vec(), &variables, &functions) ) );
            },
            _ => {
                ans = complete(tokenize(chain, &variables, &functions), &functions);
                variables.insert( ['a', 'n', 's', ' ', ' '], Bat::Val(ans) );
                println!("[Σ] {ans}");
            },
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bat {
    Val(Comp),
    Rel(BinOp),
    Begin(u16),
    End(u16),
    Comma,
    Func([char; 5]),
    Inp(u16),
    Builtin(BasicFn),
}
impl Bat {
    fn extract_val(self) -> Comp {
        match self {
            Self::Val(v) => v,
            e => panic!("attempted to extract Val from non-value {:?}", e),
        }
    }
    fn extract_rel(self) -> BinOp {
        match self {
            Self::Rel(v) => v,
            e => panic!("attempted to extract BinOp from non-operator {:?}", e),
        }
    }
    fn extract_basic(self) -> BasicFn {
        match self {
            Self::Builtin(v) => v,
            e => panic!("attempted to extract basic from non-basic {:?}", e),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

fn take_input() -> String {
    let mut stringy: String = String::new();
    std::io::stdin().read_line(&mut stringy).expect("tf u think ur doing");
    stringy
}
fn split_input(raw: String) -> Vec<String> {
    raw.replace(",", " , ")
    .replace(")", " ) ").replace("(", " ( ")
    // add whitespace for binary operations
    .split_whitespace().map(|x| x.to_string()).collect()
}
pub(crate) fn get_five(word: String) -> [char; 5] {
    let mut each = word.chars();
    let mut out: [char; 5] = [' '; 5];
    for o in 0..5 {
        match each.next() {
            Some(v) => out[o] = v,
            None => ()
        }
    }
    out
}
fn encode_one(word: String, depth: &mut u16,
    varlist: &HashMap<[char; 5], Bat>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Bat {
    match word.parse::<Comp>() {
        Ok(v) => return Bat::Val(v),
        Err(_) => (),
    };
    match word.as_str() {
        "," => return Bat::Comma,
        "(" => {*depth += 1; return Bat::Begin(*depth); },
        ")" => {*depth -= 1; return Bat::End(*depth+1); },
        "+" => return Bat::Rel(BinOp::Add),
        "-" => return Bat::Rel(BinOp::Sub),
        "*" | "×" | "·" => return Bat::Rel(BinOp::Mul),
        "/" | "÷" => return Bat::Rel(BinOp::Div),
        "^" => return Bat::Rel(BinOp::Pow),
        "exp" => return Bat::Builtin(BasicFn::Exponential),
        "sin" => return Bat::Builtin(BasicFn::Sine),
        "cos" => return Bat::Builtin(BasicFn::Cosine),
        "ln" => return Bat::Builtin(BasicFn::NaturalLog),
        "log" => return Bat::Builtin(BasicFn::LogBase),
        "asin" => return Bat::Builtin(BasicFn::ArcSine),
        "acos" => return Bat::Builtin(BasicFn::ArcCosine),
        _ => (),
    };
    if word.chars().nth(0).unwrap() == '#' {
        match word[1..].parse::<u16>() {
            Ok(v) => return Bat::Inp(v),
            Err(_) => panic!("inputs to functions like this: '#1', '#4'",)
        }
    } 
    let name: [char; 5] = get_five(word.clone());
    match varlist.get(&name) {
        Some(v) => return *v,
        None => (),
    }
    match fnlist.get(&name) {
        Some(_) => return Bat::Func(name),
        None => (),
    }
    panic!("goofball got an invalid token {}", word) // for no matches
}
fn tokenize(chain: Vec<String>, varlist: &HashMap<[char; 5], Bat>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Vec<Bat> {
    let mut depth: u16 = 0;
    let mut processed: Vec<Bat> = Vec::new();
    for word in chain {
        processed.push(encode_one(word, &mut depth, varlist, fnlist));
    };
    processed
}

fn binary_operate(operation: BinOp, first: Comp, last: Comp) -> Comp {
    match operation {
        BinOp::Add => first + last,
        BinOp::Sub => first - last,
        BinOp::Mul => first * last,
        BinOp::Div => first / last,
        BinOp::Pow => first.pow(last),
    }
}
fn find_deepest(content: Vec<Bat>) -> Option<(usize, usize)> {
    let mut start: Option<usize> = None;
    let mut finish: Option<usize> = None;
    let mut s_max: u16 = 0;
    let mut f_max: u16 = 0;
    for (indx, token) in content.iter().enumerate() {
        match *token {
            Bat::Begin(d) => {
                if d > s_max { start = Some(indx); s_max = d }
            }
            Bat::End(d) => {
                if d > f_max { finish = Some(indx); f_max = d }
            }
            _ => (),
        }
    };
    match (start, finish) {
        (Some(_), None) | (None, Some(_)) => panic!("unopened ) or unclosed ("),
        (Some(s), Some(f)) => Some((s,f)),
        (None, None) => None,
    }
}
fn bin_replace(current: &mut Vec<Bat>, indx: usize) {
    let replace: Bat = Bat::Val(
        binary_operate(current[indx].extract_rel(),
        current[indx-1].extract_val(), current[indx+1].extract_val())
    );
    current.drain(indx-1..indx+2);
    current.insert(indx-1, replace);
}
fn paren_replace(current: &mut Vec<Bat>, start: usize, end: usize) {
    let replace: Bat = order_operations(current[start+1..end].to_vec());
    current.drain(start..end+1);
    current.insert(start, replace);
}
fn func_replace(current: &mut Vec<Bat>, start: usize, end: usize,
    name: [char; 5], fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(Bat::Val(complete(expression, fnlist)))
    }
    let function: &(u16, Vec<Bat>) = fnlist.get(&name).unwrap();
    let mut working: Vec<Bat> = Vec::new();
    for entry in function.1.iter() {
        match entry {
            Bat::Inp(v) => working.push(inputs[(*v-1) as usize]),
            a => working.push(*a),
        }
    }
    current.drain(start-1..end+1);
    current.insert(start-1, Bat::Val(complete(working, fnlist)));
}
fn basic_replace(current: &mut Vec<Bat>, start: usize, end: usize) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(order_operations(expression))
    }
    let first: Comp = inputs[0].extract_val();
    let replace: Comp = match current[start-1].extract_basic() {
        BasicFn::Exponential => preset::exp(first),
        BasicFn::Sine => preset::sin(first),
        BasicFn::Cosine => preset::cos(first),
        BasicFn::NaturalLog => preset::ln(first),
        BasicFn::LogBase => preset::log(first, inputs[1].extract_val()),
        BasicFn::ArcSine => preset::asin(first),
        BasicFn::ArcCosine => preset::acos(first),
    };
    current.drain(start-1..end+1);
    current.insert(start-1, Bat::Val(replace));
}
fn order_operations(simple: Vec<Bat>) -> Bat {
    let mut shrinking: Vec<Bat> = simple.clone();
    let mut maybe: Option<usize>;
    loop {
        maybe = shrinking.clone().iter().position(|&x|
            x == Bat::Rel(BinOp::Pow));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx); continue },
            None => (),
        }
        maybe = shrinking.clone().iter().position(|&x|
        x == Bat::Rel(BinOp::Div) || x == Bat::Rel(BinOp::Mul));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx); continue },
            None => (),
        };
        maybe = shrinking.iter().position(|&x|
        x == Bat::Rel(BinOp::Sub) || x == Bat::Rel(BinOp::Add));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx); continue },
            None => (),
        };
        return shrinking[0];
    }
}
fn complete(input: Vec<Bat>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Comp {
    let mut shrinking: Vec<Bat> = input;
    loop {
        match find_deepest(shrinking.clone()) {
            None => return order_operations(shrinking).extract_val(),
            Some((s,f)) => {
                if s == 0 { paren_replace(&mut shrinking, s, f); continue };
                match shrinking[s-1] {
                    Bat::Func(name) => func_replace(&mut shrinking, s, f, name, &fnlist),
                    Bat::Builtin(_) => basic_replace(&mut shrinking, s, f),
                    Bat::Val(_) | Bat::End(_) => shrinking.insert(s, Bat::Rel(BinOp::Mul)),
                    _ => paren_replace(&mut shrinking, s, f),
                }
            },
        }
    }
}
