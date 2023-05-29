use crate::comp::Comp;
use crate::preset;
use crate::trig;
use crate::preset::BasicFn;
use std::collections::HashMap;

pub fn roll() {

    let mut variables: HashMap<[char; 5], Comp> = HashMap::new();
    let mut functions: HashMap<[char; 5], (u16, Vec<Bat>)> = HashMap::new();

    for var in preset::PRE_VAR {
        variables.insert(var.0, var.1);
    }

    functions.insert(['L', 'D', ' ', ' ', ' '], (2, preset::LIMIT_DVT.to_vec()));

    let mut ans: Comp;
    let mut chain: Vec<String>;

    loop {
        chain = split_input(take_input());

        match chain[0].as_str() {
            "def" => {
                let (finding, start): (Vec<(u16, String)>, usize) = get_inputs(chain.clone());
                replace_inputs(&mut chain, &finding);

                functions.insert( get_five(chain[1].as_str().to_string()),
                ( finding.len() as u16, tokenize(chain[start..].to_vec(), &variables, &functions) ) );
            },
            "iter" => {
                let times: u16 = match chain[1].parse::<u16>() {
                    Ok(v) => v, Err(_) => 1,
                };
                let mut expr: Vec<Bat>;
                let mut ans: Comp = Comp { r: 0.0, i: 0.0 };
                for _ in 0..times {
                    expr = tokenize(chain[2..].to_vec(), &variables, &functions);
                    ans = complete(expr, &mut variables, &functions).extract_val();
                }
                println!("[Σ] {ans}");
            }
            _ => {
                ans = complete(tokenize(chain, &variables, &functions), &mut variables, &functions).extract_val();
                variables.insert( ['a', 'n', 's', ' ', ' '], ans );
                println!("[Σ] {ans}");
            },
        };
    }
}

fn get_inputs(chain: Vec<String>) -> (Vec<(u16, String)>, usize) {
    let mut inputs: Vec<(u16, String)> = Vec::new();
    let mut indx: usize = 2;
    let mut previous: String = "(".to_string();
    let mut current: &str;
    let mut count: u16 = 0;
    loop {
        current = chain[indx].as_str();
        match current {
            ")" => { count += 1; inputs.push((count, previous)); indx += 1; break },
            "," => { count += 1; inputs.push((count, previous)) },
            _ => (),
        }
        indx += 1;
        previous = current.to_string();
    }
    (inputs, indx)
}
fn replace_inputs(chain: &mut Vec<String>, inputs: &Vec<(u16, String)>) {
    for word in chain {
        for inp in inputs {
            if word == &inp.1 { *word = format!("#{}", inp.0) }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bat {
    Val(Comp),
    Rel(BinOp),
    Begin(u16),
    End(u16),
    Comma,
    Var([char; 5], Comp),
    Func([char; 5]),
    Inp(u16),
    Builtin(BasicFn),
    Somn([char; 5]),
}
impl Bat {
    fn extract_val(self) -> Comp {
        match self {
            Self::Val(v) => v,
            Self::Var(_, v) => v,
            e => panic!("attempted to extract Val from non-value {:?}", e),
        }
    }
    fn extract_rel(self) -> BinOp {
        match self {
            Self::Rel(v) => v,
            e => panic!("attempted to extract BinOp from non-operator {:?}", e),
        }
    }
    fn extract_name(self) -> [char; 5] {
        match self {
            Self::Var(name, _) => name,
            Self::Func(name) => name,
            Self::Somn(name) => name,
            e => panic!("attempted to extract name from non-var-func {:?}", e),
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
    Assign,
}

fn take_input() -> String {
    let mut stringy: String = String::new();
    std::io::stdin().read_line(&mut stringy).expect("tf u think ur doing");
    stringy
}
fn split_input(raw: String) -> Vec<String> {
    raw.replace(",", " , ")
    .replace(")", " ) ").replace("(", " ( ")
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
fn encode_one(word: String, depth: &mut u16, varlist: &HashMap<[char; 5], Comp>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Bat {
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
        "=" => return Bat::Rel(BinOp::Assign),

        "exp" => return Bat::Builtin(BasicFn::Exponential),
        "ln" => return Bat::Builtin(BasicFn::NaturalLog),
        "log" => return Bat::Builtin(BasicFn::LogBase),

        "sin" => return Bat::Builtin(BasicFn::Sine),
        "cos" => return Bat::Builtin(BasicFn::Cosine),
        "tan" => return Bat::Builtin(BasicFn::Tangent),
        "cot" => return Bat::Builtin(BasicFn::Cotangent),
        "sec" => return Bat::Builtin(BasicFn::Secant),
        "csc" => return Bat::Builtin(BasicFn::Cosecant),

        "asin" => return Bat::Builtin(BasicFn::ArcSine),
        "acos" => return Bat::Builtin(BasicFn::ArcCosine),
        "atan" => return Bat::Builtin(BasicFn::ArcTangent),
        "acot" => return Bat::Builtin(BasicFn::ArcCotangent),
        "asec" => return Bat::Builtin(BasicFn::ArcSecant),
        "acsc" => return Bat::Builtin(BasicFn::ArcCosecant),

        "sinh" => return Bat::Builtin(BasicFn::HypSine),
        "cosh" => return Bat::Builtin(BasicFn::HypCosine),
        "tanh" => return Bat::Builtin(BasicFn::HypTangent),
        "coth" => return Bat::Builtin(BasicFn::HypCotangent),
        "sech" => return Bat::Builtin(BasicFn::HypSecant),
        "csch" => return Bat::Builtin(BasicFn::HypCosecant),

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
        Some(v) => return Bat::Var(name, *v),
        None => (),
    }
    match fnlist.get(&name) {
        Some(_) => return Bat::Func(name),
        None => (),
    }
    Bat::Somn(name) // for no matches
}
fn tokenize(chain: Vec<String>, varlist: &HashMap<[char; 5], Comp>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Vec<Bat> {
    let mut depth: u16 = 0;
    let mut processed: Vec<Bat> = Vec::new();
    for word in chain {
        processed.push(encode_one(word, &mut depth, varlist, fnlist));
    };
    processed
}

fn binary_operate(operation: BinOp, first: Bat, last: Comp, varlist: &mut HashMap<[char; 5], Comp>) -> Comp {
    match operation {
        BinOp::Add => first.extract_val() + last,
        BinOp::Sub => first.extract_val() - last,
        BinOp::Mul => first.extract_val() * last,
        BinOp::Div => first.extract_val() / last,
        BinOp::Pow => first.extract_val().pow(last),
        BinOp::Assign => { varlist.insert(first.extract_name(), last); return last },
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
fn bin_replace(current: &mut Vec<Bat>, indx: usize, varlist: &mut HashMap<[char; 5], Comp>) {
    let replace: Comp = 
    binary_operate(current[indx].extract_rel(),
    current[indx-1], current[indx+1].extract_val(), varlist);

    current.drain(indx-1..indx+2);
    current.insert(indx-1, Bat::Val(replace));
}
fn paren_replace(current: &mut Vec<Bat>, start: usize, end: usize, varlist: &mut HashMap<[char; 5], Comp>) {
    let replace: Bat = order_operations(current[start+1..end].to_vec(), varlist);
    current.drain(start..end+1);
    current.insert(start, replace);
}
fn func_replace(current: &mut Vec<Bat>, start: usize, end: usize,
    name: [char; 5], varlist: &mut HashMap<[char; 5], Comp>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(complete(expression, varlist, fnlist));
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
    current.insert(start-1, complete(working, varlist, fnlist));
}
fn basic_replace(current: &mut Vec<Bat>, start: usize, end: usize, varlist: &mut HashMap<[char; 5], Comp>) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(order_operations(expression, varlist))
    }
    let first: Comp = inputs[0].extract_val();
    let replace: Comp = match current[start-1].extract_basic() {

        BasicFn::Exponential => preset::exp(first),
        BasicFn::NaturalLog => preset::ln(first),
        BasicFn::LogBase => preset::log(first, inputs[1].extract_val()),

        BasicFn::Sine => trig::sin(first),
        BasicFn::Cosine => trig::cos(first),
        BasicFn::Tangent => trig::tan(first),
        BasicFn::Cotangent => trig::cot(first),
        BasicFn::Secant => trig::sec(first),
        BasicFn::Cosecant => trig::csc(first),
    
        BasicFn::ArcSine => trig::asin(first),
        BasicFn::ArcCosine => trig::acos(first),
        BasicFn::ArcTangent => trig::atan(first),
        BasicFn::ArcCotangent => trig::acot(first),
        BasicFn::ArcSecant => trig::asec(first),
        BasicFn::ArcCosecant => trig::acsc(first),

        BasicFn::HypSine => trig::sinh(first),
        BasicFn::HypCosine => trig::cosh(first),
        BasicFn::HypTangent => trig::tanh(first),
        BasicFn::HypCotangent => trig::coth(first),
        BasicFn::HypSecant => trig::sech(first),
        BasicFn::HypCosecant => trig::csch(first),

    };
    current.drain(start-1..end+1);
    current.insert(start-1, Bat::Val(replace));
}

fn order_operations(simple: Vec<Bat>, varlist: &mut HashMap<[char; 5], Comp>) -> Bat {
    let mut shrinking: Vec<Bat> = simple.clone();
    let mut maybe: Option<usize>;
    loop {
        maybe = shrinking.clone().iter().position(|&x|
            x == Bat::Rel(BinOp::Pow));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx, varlist); continue },
            None => (),
        }
        maybe = shrinking.clone().iter().position(|&x|
        x == Bat::Rel(BinOp::Div) || x == Bat::Rel(BinOp::Mul));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx, varlist); continue },
            None => (),
        };
        maybe = shrinking.clone().iter().position(|&x|
        x == Bat::Rel(BinOp::Sub) || x == Bat::Rel(BinOp::Add));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx, varlist); continue },
            None => (),
        };
        maybe = shrinking.iter().position(|&x|
        x == Bat::Rel(BinOp::Assign));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx, varlist); continue },
            None => (),
        }
        return shrinking[0];
    }
}
fn complete(input: Vec<Bat>, varlist: &mut HashMap<[char; 5], Comp>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> Bat {
    let mut shrinking: Vec<Bat> = input;
    loop {
        match find_deepest(shrinking.clone()) {
            None => return order_operations(shrinking, varlist),
            Some((s,f)) => {
                if s == 0 { paren_replace(&mut shrinking, s, f, varlist); continue };
                match shrinking[s-1] {
                    Bat::Func(name) => func_replace(&mut shrinking, s, f, name, varlist, &fnlist),
                    Bat::Builtin(_) => basic_replace(&mut shrinking, s, f, varlist),
                    Bat::Val(_) | Bat::End(_) => shrinking.insert(s, Bat::Rel(BinOp::Mul)),
                    _ => paren_replace(&mut shrinking, s, f, varlist),
                }
            },
        }
    }
}