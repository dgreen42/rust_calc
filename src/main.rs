use std::num::ParseIntError;
use std::str::from_utf8;

struct Number {
    content: i32,
}

struct Modifier {
    content: String,
}

struct Item {
    content: String,
}

impl Default for Item {
    fn default() -> Self {
        Item {content: String::new()}
    }
}

struct Term {
    item: Item,
    group: Group,
}

impl Default for Term {
    fn default() -> Self {
        Term {item: Item::default(), group: Group::default()}
    }
}


struct Expression {
    term: Term,
    next: Expression,
}

impl Default for Expression {
    fn default() -> Self {
        Expression {term: Term::default(), next: Expression::default()}
    }
}


struct Group {
    content: Expression,
}

impl Default for Group {
    fn default() -> Self {
        Group {content: Expression::default()}
    }
}

fn is_number(n: &str) -> Option<i32> {
    let result: Option<i32> = match n.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_e) => None,
    };

    return result;
}

fn is_modifier(m: &str) -> Option<String> {
    let result: Option<String> = match m {
        "+" => Some(String::from("+")),
        "-" => Some(String::from("-")),
        "*" => Some(String::from("*")),
        "/" => Some(String::from("/")),
        _ => None,
    };
    result
}

fn number(exp: String) -> (Option<Number>, String) {
    let mut exp_next = exp.chars();
    let number: Option<Number> = match is_number(&exp_next.next().unwrap().to_string()) {
        Some(number) => Some(Number {content:number}),
        None => None,
    };
    let mut after = String::new();
    if number.is_some() {
        for _idx in 0..exp.len() - 1 {
            let next_char = exp_next.next();
            after.push_str(&next_char.unwrap().to_string());
        }
    }
    
    return (number, after)
}

fn modifier(exp: String) -> (Option<Modifier>, String) {
    let mut exp_next = exp.chars();
    let modifier: Option<Modifier> = match is_modifier(&exp_next.next().unwrap().to_string()) {
        Some(modi) => Some(Modifier {content:modi}),
        None => None,
    };
    let mut after = String::new();
    if modifier.is_some() {
        for _idx in 0..exp.len() - 1 {
            let next_char = exp_next.next();
            after.push_str(&next_char.unwrap().to_string());
        }
    }

    return (modifier, after)
}

fn expression(exp: String) -> (Option<Expression>, String) {
    if term(exp).0.is_some() {
        let trm = term(exp);
        let after_trm = trm.1;
        if expression(after_trm).is_some() {

        }
    }
}

fn group(exp: String) -> (Option<Group>, String) {}

fn item(exp: String) -> (Option<Item>, String) {}

fn term(exp: String) -> (Option<Term>, String) {}

fn main() {
    let simple: String = String::from("1+2");



}

