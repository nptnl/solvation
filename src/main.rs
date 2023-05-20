fn main() {
    loop {
        let dong = order_operations(tokenize(split_input(take_input()))).extract_val();
        println!("{:?}", dong);
    }
}

fn take_input() -> String {
    let mut stringy: String = String::new();
    std::io::stdin().read_line(&mut stringy);
    stringy
}
fn split_input(raw: String) -> Vec<String> {
    raw.replace(",", " , ")
    .replace(")", " ) ").replace("(", " ( ")
    // add whitespace for binary operations
    .split_whitespace().map(|x| x.to_string()).collect()
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Bat {
    Val(i32),
    Rel(BinOp),
    Begin(u16),
    End(u16),
    Comma,
    Func([char; 5]),
}
impl Bat {
    fn extract_val(self) -> i32 {
        match self {
            Self::Val(v) => v,
            _ => panic!("attempted to extract Val from non-value Bat"),
        }
    }
    fn extract_rel(self) -> BinOp{
        match self {
            Self::Rel(v) => v,
            _ => panic!("attempted to extract BinOp from non-operator Bat"),
        }
    }
}
fn get_five(word: String) -> [char; 5] {
    let mut le: usize = word.len();
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
fn encode_one(word: String, mut depth: u16) -> Bat {
    match word.parse::<i32>() {
        Ok(v) => return Bat::Val(v),
        Err(_) => (),
    };
    match word.as_str() {
        "," => return Bat::Comma,
        "(" => {depth += 1; return Bat::Begin(depth-1); },
        ")" => {depth -= 1; return Bat::End(depth); },
        "+" => return Bat::Rel(BinOp::Add),
        "-" => return Bat::Rel(BinOp::Sub),
        "*" => return Bat::Rel(BinOp::Mul),
        "/" => return Bat::Rel(BinOp::Div),
        v => panic!("invalid token {}", v),
    };
    let name: [char; 5] = get_five(word);
    // look for existing functions or variables
    return Bat::Func(name);
}
fn tokenize(chain: Vec<String>) -> Vec<Bat> {
    let mut depth: u16 = 0;
    let mut processed: Vec<Bat> = Vec::new();
    for word in chain {
        processed.push(encode_one(word, depth));
    };
    processed
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
fn binary_operate(operation: BinOp, first: i32, last: i32) -> i32 {
    match operation {
        BinOp::Add => first + last,
        BinOp::Sub => first - last,
        BinOp::Mul => first * last,
        BinOp::Div => first / last,
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