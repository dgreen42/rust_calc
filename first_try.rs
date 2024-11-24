
use std::num::ParseIntError;
use std::str::from_utf8;

struct Number {
    content: i32,
}

struct Modifier {
    content: String,
}

struct Item {
    content_char: String,
    content_grp: Group,
}

struct Term {
    item: Item,
    group: Group,
}

struct Expression {
    term: Term,
    next: Option<Expression>,
}

struct Group {
    content: Expression,
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
        return (number, after)
    } else {
        return (None, after) // come back to this after needs to be generated
    }

    
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
        return (modifier, after)
    } else {
        return (None, after) // come back to this after needs to be generated
    }

}

fn expression(exp: String) -> (Option<Expression>, String) {
    if term(exp).0.is_some() {
        let trm = term(exp);
        let after_trm = trm.1;
        if expression(after_trm).0.is_some() {
            let expr = expression(after_trm);
            let fin_expr = Expression { term: trm.0, next: expr.0}; 
            return (Some(fin_expr), expr.1)
        } else {
            let fin_expr = Expression {term: trm.0, next: None};
            return (Some(fin_expr), trm.1)
        }
    } else {
        return (None, after)
    }
}

fn group(exp: String) -> (Option<Group>, String) {
    let exp_next = exp.chars();
    if exp_next.next().unwrap().to_string() == "(" {
        let after_bracket = String::new();
        while exp_next.next().unwrap().to_string() != ")" {
            let next_char = exp_next.next();
            after_bracket.push_str(&next_char.unwrap().to_string());
        }
        if expression(after_bracket).0.is_some() {
            if exp_next.next().unwrap().to_string() == ")" {
                let grp = Group { content: after_bracket };
                let after = String::new();
                for _idx in exp.len() - after_bracket - 2 {
                    let next_char = exp_next.next();
                    after.push_str(&next_char.unwrap().to_string());
                }
                return (Some(Group), after)
            } else {
                return (None, after)
            }
        } else {
            return (None, after)
        }
    } else {
        return (None, after)
    }
}

fn item(exp: String) -> (Option<Item>, String) {
    if number(exp).0.is_some() {
        let (parsed_number, after) = number(exp);
        let itm = Item { content: parsed_number.unwrap().content.to_string() };
        return (Some(Item), after)
    } 
    if group(exp).0.is_some() {
        let (parsed_number, after) = group(exp);
        let itm = Item { content: parsed_number.unwrap() };
        return (Some(Item), after)
    }
    

}

fn term(exp: String) -> (Option<Term>, String) {}

fn main() {
    let simple: String = String::from("1+2");



}

