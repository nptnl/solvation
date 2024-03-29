use crate::math::comp::Comp;
use crate::math::rat::Rat;
use crate::math::{prim, trig};
use crate::preset;
use crate::math::BasicFn;
use std::collections::HashMap;

pub fn roll() {

    let mut variables: HashMap<[char; 5], Type> = HashMap::new();
    let mut functions: HashMap<[char; 5], (u16, Vec<Bat>)> = HashMap::new();

    for var in preset::PRE_VAR {
        variables.insert(var.0, Type::C(var.1));
    }
    
    let mut pre = preset::PRE_DO.iter();
    let mut chain: Vec<String>;
    let mut cur;

    loop {
        cur = pre.next();
        chain = match cur {
            Some(v) => split_input(v.to_string()),
            None => split_input(take_input()),
        };

        match chain[0].as_str() {
            "def" => {
                let (finding, start): (Vec<(u16, String)>, usize) = get_inputs(chain.clone());
                replace_inputs(&mut chain, &finding);

                functions.insert( get_five(chain[1].as_str().to_string()),
                ( finding.len() as u16, tokenize(chain[start..].to_vec(), &variables, &functions) ) );

                println!("[Σ] Done");
            },
            _ => {
                let ans = complete(tokenize(chain, &variables, &functions), &mut variables, &functions);
                if let Bat::Val(_) | Bat::Var(_, _) = ans {
                    variables.insert( ['a', 'n', 's', ' ', ' '], ans.extract_val() );
                }
                match ans {
                    Bat::Nomn => (),
                    Bat::Break => (),
                    _ => println!("[Σ] {ans}"),
                }
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
    Val(Type),
    Var([char; 5], Type),
    Somn([char; 5]),
    Func([char; 5]),
    Builtin(BasicFn),
    Inp(u16),

    Rel(BinOp),
    Assign,
    Increment,
    
    Begin(u16),
    End(u16),
    LoopBegin(u16),
    LoopEnd(u16),
    Comma,
    Condition,
    Break,
    Nomn,
}
impl Bat {
    fn extract_val(self) -> Type {
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
            e => panic!("attempted to extract name from non-somn {:?}", e),
        }
    }
    fn extract_basic(self) -> BasicFn {
        match self {
            Self::Builtin(v) => v,
            e => panic!("attempted to extract basic from non-basic {:?}", e),
        }
    }
}
impl std::fmt::Display for Bat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bat::Val(v) => write!(f, "{}", *v),
            Bat::Var(_, v) => write!(f, "{}", *v),
            Bat::Break => write!(f, "BREAK"),
            _ => write!(f, "Done"),
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

    Equal,
    Less,
    Greater,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Type {
    C(Comp),
    Q(Rat),
    N(u16),
    B(bool),
}
impl Type {
    fn get_bool(self) -> bool {
        match self {
            Self::B(v) => v,
            e => panic!("attempted to extract boolean from non-bool {:?}", e),
        }
    }
    fn get_indx(self) -> u16 {
        match self {
            Self::N(v) => v,
            e => panic!("attempted to extract indx from non-nat {:?}", e),
        }
    }
    fn get_comp(self) -> Comp {
        match self {
            Self::C(v) => v,
            e => panic!("attempted to extract comp from non-comp {:?}", e),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        if slice.chars().nth(0).unwrap() == '`' {
            match slice[1..].parse::<u16>() {
                Ok(v) => return Ok(Self::N(v)),
                Err(_) => return Err(()),
            }
        }
        if let Ok(v) = slice.parse::<bool>() { return Ok(Self::B(v)) }
        if let Ok(v) = slice.parse::<Rat>() { return Ok(Self::Q(v)) }
        match slice.parse::<Comp>() {
            Ok(v) => Ok(Self::C(v)),
            Err(e) => Err(e),
        }
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::C(v) => write!(f, "{}", v),
            Self::Q(v) => write!(f, "{}", v),
            Self::N(v) => write!(f, "{}", v),
            Self::B(v) => write!(f, "{}", v),
        }
    }
}

fn take_input() -> String {
    let mut stringy: String = String::new();
    std::io::stdin().read_line(&mut stringy).expect("cannot read input");
    stringy
}
fn split_input(raw: String) -> Vec<String> {
    raw.replace(",", " , ")
    .replace(")", " ) ").replace("(", " ( ")
    .replace("[", " [ ").replace("]", " ] ")
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
fn encode_one(
    word: String,
    depth: &mut u16,
    varlist: &HashMap<[char; 5], Type>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>,
) -> Bat {
    match word.parse::<Type>() {
        Ok(v) => return Bat::Val(v),
        Err(_) => (),
    };
    match word.as_str() {

        "," => return Bat::Comma,
        "(" => {*depth += 1; return Bat::Begin(*depth); },
        ")" => {*depth -= 1; return Bat::End(*depth+1); },
        "[" => {*depth += 1; return Bat::LoopBegin(*depth); },
        "]" => {*depth -= 1; return Bat::LoopEnd(*depth+1); },
        "+" => return Bat::Rel(BinOp::Add),
        "-" => return Bat::Rel(BinOp::Sub),
        "*" | "×" | "·" => return Bat::Rel(BinOp::Mul),
        "/" | "÷" => return Bat::Rel(BinOp::Div),
        "^" => return Bat::Rel(BinOp::Pow),

        "=" => return Bat::Assign,
        "++" => return Bat::Increment,
        ":" => return Bat::Condition,
        "==" => return Bat::Rel(BinOp::Equal),
        "<" => return Bat::Rel(BinOp::Less),
        ">" => return Bat::Rel(BinOp::Greater),
        "∇" => return Bat::Break,

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

        "asinh" => return Bat::Builtin(BasicFn::ArcHypSine),
        "acosh" => return Bat::Builtin(BasicFn::ArcHypCosine),
        "atanh" => return Bat::Builtin(BasicFn::ArcHypTangent),
        "acoth" => return Bat::Builtin(BasicFn::ArcHypCotangent),
        "asech" => return Bat::Builtin(BasicFn::ArcHypSecant),
        "acsch" => return Bat::Builtin(BasicFn::ArcHypCosecant),

        "sqrt" | "√" => return Bat::Builtin(BasicFn::Sqrt),

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
fn tokenize(
    chain: Vec<String>,
    varlist: &HashMap<[char; 5], Type>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>
) -> Vec<Bat> {
    let mut depth: u16 = 0;
    let mut processed: Vec<Bat> = Vec::new();
    for word in chain {
        processed.push(encode_one(word, &mut depth, varlist, fnlist));
    };
    processed
}

fn binary_operate( operation: BinOp, first: Bat, last: Bat ) -> Bat {
    match (first.extract_val(), last.extract_val()) {
        (Type::C(v1), Type::C(v2)) => {
            return match operation {
                BinOp::Add => Bat::Val(Type::C( v1 + v2 )),
                BinOp::Sub => Bat::Val(Type::C( v1 - v2 )),
                BinOp::Mul => Bat::Val(Type::C( v1 * v2 )),
                BinOp::Div => Bat::Val(Type::C( v1 / v2 )),
                BinOp::Pow => Bat::Val(Type::C( v1.pow(v2) )),

                BinOp::Equal => Bat::Val(Type::B(v1 == v2)),
                BinOp::Less => Bat::Val(Type::B(v1 < v2)),
                BinOp::Greater => Bat::Val(Type::B(v1 > v2)),
            }
        },
        (Type::N(v1), Type::N(v2)) => {
            return match operation {
                BinOp::Add => Bat::Val(Type::N( v1 + v2 )),
                BinOp::Sub => Bat::Val(Type::N( v1 - v2 )),
                BinOp::Mul => Bat::Val(Type::N( v1 * v2 )),
                BinOp::Div => Bat::Val(Type::N( v1 / v2 )),
                BinOp::Pow => Bat::Val(Type::N( v1.pow(v2 as u32) )),

                BinOp::Equal => Bat::Val(Type::B(v1 == v2)),
                BinOp::Less => Bat::Val(Type::B(v1 < v2)),
                BinOp::Greater => Bat::Val(Type::B(v1 > v2)),
            }
        },
        (Type::Q(v1), Type::Q(v2)) => {
            return match operation {
                BinOp::Add => Bat::Val(Type::Q( v1 + v2 )),
                BinOp::Sub => Bat::Val(Type::Q( v1 - v2 )),
                BinOp::Mul => Bat::Val(Type::Q( v1 * v2 )),
                BinOp::Div => Bat::Val(Type::Q( v1 / v2 )),
                BinOp::Pow => Bat::Nomn,
                
                BinOp::Equal => Bat::Val(Type::B(v1 == v2)),
                BinOp::Less => Bat::Val(Type::B(v1 < v2)),
                BinOp::Greater => Bat::Val(Type::B(v1 > v2)),
            }
        }
        _ => panic!("bro is trying to use operators on non-numbers smh"),
    };
}
fn find_deepest(content: Vec<Bat>) -> Option<(usize, usize, bool)> {
    let mut start: Option<usize> = None;
    let mut finish: Option<usize> = None;
    let mut s_max: u16 = 0;
    let mut f_max: u16 = 0;
    let mut loping: bool = false;
    for (indx, token) in content.iter().enumerate() {
        match *token {
            Bat::Begin(d) => {
                if loping { () }
                else if d > s_max { start = Some(indx); s_max = d }
            },
            Bat::End(d) => {
                if loping { () }
                else if d > f_max { finish = Some(indx); f_max = d }
            },
            Bat::LoopBegin(d) => {
                if d > s_max { start = Some(indx); s_max = d; loping = true; }
            },
            Bat::LoopEnd(d) => {
                if d > f_max { finish = Some(indx); f_max = d; loping = true; }
            },
            _ => (),
        }
    };
    match (start, finish) {
        (Some(_), None) | (None, Some(_)) => panic!("unopened ) or unclosed ("),
        (None, None) => None,
        (Some(s), Some(f)) => {
            let looping: bool =
            if let Bat::LoopBegin(_) = content[s] { true } else { false };
            Some((s, f, looping))
        },
    }
}
fn bin_replace(
    current: &mut Vec<Bat>,
    indx: usize,
) {
    let replace: Bat = 
    binary_operate(current[indx].extract_rel(), current[indx-1], current[indx+1]);
    current.drain(indx-1..indx+2);
    current.insert(indx-1, replace);
}
fn paren_replace(
    current: &mut Vec<Bat>,
    start: usize, end: usize,
    varlist: &mut HashMap<[char; 5], Type>,
) {
    let replace: Bat = order_operations(current[start+1..end].to_vec(), varlist);
    current.drain(start..end+1);
    current.insert(start, replace);
}
fn func_replace(
    current: &mut Vec<Bat>,
    start: usize, end: usize, name: [char; 5],
    varlist: &mut HashMap<[char; 5], Type>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>
) {
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
fn basic_replace(
    current: &mut Vec<Bat>,
    start: usize, end: usize,
    varlist: &mut HashMap<[char; 5], Type>,
) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(order_operations(expression, varlist))
    }
    let first: Comp = inputs[0].extract_val().get_comp();
    let replace: Comp = match current[start-1].extract_basic() {

        BasicFn::Exponential => prim::exp(first),
        BasicFn::NaturalLog => prim::ln(first),
        BasicFn::LogBase => prim::log(first, inputs[1].extract_val().get_comp()),

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

        BasicFn::ArcHypSine => trig::asinh(first),
        BasicFn::ArcHypCosine => trig::acosh(first),
        BasicFn::ArcHypTangent => trig::atanh(first),
        BasicFn::ArcHypCotangent => trig::acoth(first),
        BasicFn::ArcHypSecant => trig::asech(first),
        BasicFn::ArcHypCosecant => trig::acsch(first),

        BasicFn::Sqrt => crate::math::comp::comp_sqrt(first),

    };
    current.drain(start-1..end+1);
    current.insert(start-1, Bat::Val(Type::C(replace)));
}
fn conditional_replace(
    current: &mut Vec<Bat>,
    start: usize, end: usize,
    varlist: &mut HashMap<[char; 5], Type>,
) {
    let split: Vec<Vec<Bat>> = current[start+1..end].split(|&x| x == Bat::Comma)
    .map(|x| x.to_vec()).collect();
    let mut inputs: Vec<Bat> = Vec::new();
    for expression in split {
        inputs.push(order_operations(expression, varlist))
    }
    let replace: Bat;
    if inputs[0].extract_val().get_bool() { replace = inputs[1] }
    else { replace = Bat::Nomn }
    current.drain(start-1..end+1);
    current.insert(start-1, replace);
}

fn do_assign(
    current: &mut Vec<Bat>,
    indx: usize,
    varlist: &mut HashMap<[char; 5], Type>,
) {
    varlist.insert(current[indx-1].extract_name(), current[indx+1].extract_val());
    current.drain(indx-1..indx+2);
    current.insert(indx-1, Bat::Nomn);
}
fn do_increment(
    current: &mut Vec<Bat>,
    indx: usize,
    varlist: &mut HashMap<[char; 5], Type>,
) {
    varlist.insert(current[indx-1].extract_name(), Type::N(current[indx-1].extract_val().get_indx() + 1u16));
    current.drain(indx-1..indx+1);
    current.insert(indx-1, Bat::Nomn);
}

fn order_operations(
    simple: Vec<Bat>,
    varlist: &mut HashMap<[char; 5], Type>,
) -> Bat {
    let mut shrinking: Vec<Bat> = simple.clone();
    let mut maybe: Option<usize>;
    loop {
        maybe = shrinking.clone().iter().position(|&x| x == Bat::Rel(BinOp::Pow));
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
        maybe = shrinking.clone().iter().position(|&x|
        x == Bat::Rel(BinOp::Sub) || x == Bat::Rel(BinOp::Add));
        match maybe {
            Some(indx) => { bin_replace(&mut shrinking, indx); continue },
            None => (),
        };
        maybe = shrinking.clone().iter().position(|&x| x == Bat::Assign);
        match maybe {
            Some(indx) => { do_assign(&mut shrinking, indx, varlist); continue },
            None => (),
        }
        maybe = shrinking.clone().iter().position(|&x| x == Bat::Increment);
        match maybe {
            Some(indx) =>  { do_increment(&mut shrinking, indx, varlist); continue },
            None => (),
        }
        maybe = shrinking.clone().iter().position(|&x|
        x == Bat::Rel(BinOp::Equal) || x == Bat::Rel(BinOp::Less) || x == Bat::Rel(BinOp::Greater));
        match maybe {
            Some(indx) =>  { bin_replace(&mut shrinking, indx); continue },
            None => (),
        }

        let mut indx: usize = 0;
        loop {
            if indx == shrinking.len() { return Bat::Nomn }
            if shrinking[indx] != Bat::Nomn && shrinking[indx] != Bat::Comma { return shrinking[indx] }
            indx += 1;
        }
    }
}
fn complete(
    input: Vec<Bat>,
    varlist: &mut HashMap<[char; 5], Type>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>,
) -> Bat {
    let mut shrinking: Vec<Bat> = input;
    loop {
        match find_deepest(shrinking.clone()) {
            None => return order_operations(shrinking, varlist),
            Some((s, f, looping)) => {
                if looping { exec_iter(&mut shrinking, s, f, varlist, fnlist); continue };
                if s == 0 { paren_replace(&mut shrinking, s, f, varlist); continue };
                match shrinking[s-1] {
                    Bat::Func(name) => func_replace(&mut shrinking, s, f, name, varlist, &fnlist),
                    Bat::Builtin(_) => basic_replace(&mut shrinking, s, f, varlist),
                    Bat::Val(_) | Bat::End(_) => shrinking.insert(s, Bat::Rel(BinOp::Mul)),
                    Bat::Condition => conditional_replace(&mut shrinking, s, f, varlist),
                    _ => paren_replace(&mut shrinking, s, f, varlist),
                }
            },
        }
    }
}

fn exec_iter(
    current: &mut Vec<Bat>,
    start: usize, end: usize,
    varlist: &mut HashMap<[char; 5], Type>,
    fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>,
) {
    let mut expr: Bat;
    println!("processing...");
    loop {
        update_vars(current, varlist);
        expr = complete(current[start+1..end].to_vec(), varlist, fnlist);
        if expr == Bat::Break {
            current.drain(start..end+1);
            current.insert(start, expr);
            break
        }
    }
    println!("complete!");
}
fn update_vars(
    current: &mut Vec<Bat>,
    varlist: &mut HashMap<[char; 5], Type>
) {
    for (indx, each) in current.clone().iter().enumerate() {
        match each {
            Bat::Var(name, _) => current[indx] = Bat::Var(*name, *varlist.get(name).unwrap()),
            _ => (),
        }
    }
}