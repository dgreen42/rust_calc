use std::str::Chars;

struct Expression {
    lhs: String,
    opp: String,
    rhs: String, 
}

fn is_number(n: &str) -> Option<i32> {
    let result: Option<i32> = match n.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_e) => None,
    };

    return result;
}

fn number_length(mut iter: Chars) -> i32 {
    let mut count = 0;
    let size = iter.size_hint().1.unwrap();
    println!("{:?}", iter);
    while !is_opperation(&iter.next().unwrap().to_string()).is_some() | (count == size as i32) {
        count += 1;
    }
    return count
}

fn is_opperation(m: &str) -> Option<String> {
    let result: Option<String> = match m {
        "+" => Some(String::from("+")),
        "-" => Some(String::from("-")),
        "*" => Some(String::from("*")),
        "/" => Some(String::from("/")),
        _ => None,
    };
    result
}

fn evaluate(expr: Expression) -> Option<i32> {
    let lhs = expr.lhs.parse::<i32>().unwrap();
    let rhs = expr.rhs.parse::<i32>().unwrap();
    let result = match expr.opp.as_ref() {
        "+" => Some(lhs + rhs),
        "-" => Some(lhs - rhs),
        "*" => Some(lhs * rhs),
        "/" => Some(lhs / rhs),
        _ => None,
    };
    return result
}

fn expression(exp: String) -> Option<i32> {
    println!("expression: {}", exp);
    let mut next = exp.chars();

    let number_length = number_length(next.clone());
    let mut first_number = String::new();
    for _i in 0..number_length {
        first_number.push(next.next().unwrap());
    }
    println!("first_num: {}", first_number);

    if is_number(&first_number).is_some() {
        let second_char = next.next().unwrap().to_string();
        println!("opp: {}", second_char);

        if is_opperation(&second_char).is_some() {
            let opp = is_opperation(&second_char).unwrap();
            let exp_priority = match opp.as_ref() {
                "+" => 0,
                "-" => 0,
                "*" => 1,
                "/" => 1,
                _ => 2,
            };

            if exp_priority == 1 {
                return evaluate_high_priority(first_number, second_char, next, exp.len());
            } else if exp_priority == 0 {
                return evaluate_low_priority(first_number, second_char, next, exp.len());
            } else if exp_priority == 2 {
                println!("Some how the opperation is not an opperation?");
                return None
            } else {
                return None
            }
        } else {
            return None
        }
    } else {
        return None
    }
}

fn evaluate_high_priority(first_char: String, second_char: String, mut number_iter: Chars, length: usize) -> Option<i32> {

    let number_length = number_length(number_iter.clone());
    let mut third_number = String::new();
    for _i in 0..number_length {
        third_number.push(number_iter.next().unwrap());
    }
    println!("third_num high priority: {}", third_number);

    if is_number(&third_number).is_some() {
        let mut after = String::new();
        for _ch in 0..length - 3 {
            let next_num = number_iter.next().unwrap().to_string();
            after.push_str(&next_num);
        }

        if after.is_empty() {
            let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number};
            return Some(evaluate(expr).unwrap())

        } else {
            let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number};
            let result = format!("{}{}", evaluate(expr).unwrap(), after);
            println!("result &: {}", result);
            let final_result = expression(result);
            return Some(final_result.unwrap())
        }
    } else {
        return None
    }
}

fn evaluate_low_priority(first_char: String, second_char: String, mut number_iter: Chars, length: usize) -> Option<i32> {

    let number_length = number_length(number_iter.clone());
    let mut third_number = String::new();
    for _i in 0..number_length {
        third_number.push(number_iter.next().unwrap());
    }

    println!("third_num low priority: {}", third_number);
    let mut after = String::new();
    for _ch in 0..length - 3 { // 3 cause that is the size of an expression
        let next_num = number_iter.next().unwrap().to_string();
        after.push_str(&next_num);
    }
    //this is where the final expression is evaluated, lowest opperation priority and
    //nothing after

    if after.is_empty() {
        println!("after is empty: {}", after);
        let final_expr = Expression {lhs: first_char, opp: second_char, rhs: third_number};
        return Some(evaluate(final_expr).unwrap())

    //low priority evaluates next expression before evaluating the final expression
    } else {
        third_number.push_str(&after);
        println!("after is not empty: {}", third_number);
        let result = expression(third_number); //here
        println!("third_num after recusion: {}", result.unwrap().to_string());
        let expr = Expression {lhs: first_char, opp: second_char, rhs: result.unwrap().to_string()};
        return Some(evaluate(expr).unwrap())

    }
}


fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let simple4: String = String::from("6/2+2");
    let simple5: String = String::from("6/2+2*10");
    let bad: String = String::from("this");

    let result = expression(simple5);
    println!("result: {}", result.unwrap());

}

