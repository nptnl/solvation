use std::collections::HashMap;

fn main() {

    let mut variables: HashMap<[char; 5], Bat> = HashMap::new();
    let mut functions: HashMap<[char; 5], (u16, Vec<Bat>)> = HashMap::new();
    let mut ans: i32;

    loop {
        let chain = split_input(take_input());

        match chain[0].as_str() {
            "var" => {
                variables.insert( get_five(chain[1].as_str().to_string()),
                Bat::Val(complete(tokenize(chain[2..].to_vec(), &variables, &functions), &functions)) );
            },
            "def" => {
                functions.insert( get_five(chain[1].as_str().to_string()),
                ( chain[2].parse::<u16>().unwrap(), tokenize(chain[3..].to_vec(), &variables, &functions) ) );
            },
            // put function defining here
            _ => {
                ans = complete(tokenize(chain, &variables, &functions), &functions);
                variables.insert( ['a', 'n', 's', ' ', ' '], Bat::Val(ans) );
                println!("[Σ] {ans}");
            },
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bat {
    Val(i32),
    Rel(BinOp),
    Begin(u16),
    End(u16),
    Comma,
    Func([char; 5]),
    Inp(u16),
}
impl Bat {
    fn extract_val(self) -> i32 {
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
    fn extract_name(self) -> [char; 5] {
        match self {
            Self::Func(v) => v,
            e => panic!("attempted to extract name from non-function {:?}", e),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
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
fn get_five(word: String) -> [char; 5] {
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
    match word.parse::<i32>() {
        Ok(v) => return Bat::Val(v),
        Err(_) => (),
    };
    match word.as_str() {
        "," => return Bat::Comma,
        "(" => {*depth += 1; return Bat::Begin(*depth); },
        ")" => {*depth -= 1; return Bat::End(*depth+1); },
        "+" => return Bat::Rel(BinOp::Add),
        "-" => return Bat::Rel(BinOp::Sub),
        "*" => return Bat::Rel(BinOp::Mul),
        "/" => return Bat::Rel(BinOp::Div),
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

fn binary_operate(operation: BinOp, first: i32, last: i32) -> i32 {
    match operation {
        BinOp::Add => first + last,
        BinOp::Sub => first - last,
        BinOp::Mul => first * last,
        BinOp::Div => first / last,
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
fn order_operations(simple: Vec<Bat>) -> Bat {
    let mut shrinking: Vec<Bat> = simple.clone();
    let mut maybe: Option<usize>;
    loop {
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
fn complete(input: Vec<Bat>, fnlist: &HashMap<[char; 5], (u16, Vec<Bat>)>) -> i32 {
    let mut shrinking: Vec<Bat> = input;
    loop {
        match find_deepest(shrinking.clone()) {
            None => return order_operations(shrinking).extract_val(),
            Some((s,f)) => {
                if s == 0 { paren_replace(&mut shrinking, s, f); continue };
                let name: [char; 5] = shrinking[s-1].extract_name();
                match fnlist.get(&name) {
                    Some(_) => func_replace(&mut shrinking, s, f, name, &fnlist),
                    None => paren_replace(&mut shrinking, s, f),
                }
            },
        }
    }
}

// g