use std::str::Chars;

pub mod test;

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
    while !is_opperation(&iter.next().unwrap().to_string()).is_some() {
        count += 1;
        if count == size as i32 {
            break
        }
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
        let n = next.next().unwrap();
        first_number.push(n);
    }

    println!("first_num: {}", first_number);

    if first_number.contains("(") {
        println!("Paren");
        first_number = evaluate_paren(&first_number, &next).unwrap();
        while next.next().unwrap().to_string() != ")" {
            next.next();
        }
    }
    if is_number(&first_number).is_some() {
        let second_char = next.next().unwrap().to_string();
        println!("opp: {}", second_char);

        if is_opperation(&second_char).is_some() {
            let opp = is_opperation(&second_char).unwrap();
            let opp_priority = match opp.as_ref() {
                "+" => 0,
                "-" => 0,
                "*" => 1,
                "/" => 1,
                _ => 2,
            };

            if opp_priority == 1 {
                return evaluate_high_priority(first_number, second_char, next, exp.len());
            } else if opp_priority == 0 {
                return evaluate_low_priority(first_number, second_char, next, exp.len());
            } else if opp_priority == 2 {
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

fn evaluate_paren(first_char: &str, number_iter: &Chars) -> Option<String> {
    let mut first_char = first_char.to_string();
    first_char.push_str(number_iter.as_str());
    let mut new_expression = String::new();
    for ch in first_char.chars() {
        if ch == '(' {
            continue
        } else if ch != ')' {
            new_expression.push_str(&ch.to_string());
        } else {
            break
        }
    }

    println!("Expression in paren: {}", new_expression);
    let result = expression(new_expression);
    return Some(result.unwrap().to_string())

    }

fn evaluate_high_priority(first_char: String, second_char: String, mut number_iter: Chars, length: usize) -> Option<i32> {

    let expr_lens = get_expression_length(&first_char, &number_iter).unwrap();
    let mut third_number = String::new();
    for _i in 0..expr_lens.2 {
        third_number.push(number_iter.next().unwrap());
    }
    println!("third_num high priority: {}", third_number);

    if third_number.contains("(") {
        println!("Paren");
        third_number = evaluate_paren(&third_number, &number_iter).unwrap();
        while number_iter.next().unwrap().to_string() != ")" {
            number_iter.next();
        }
    }

    if is_number(&third_number).is_some() {
        let mut after = String::new();
        println!("Full Expression Length: {}", length);
        println!("Expression Length: {}", expr_lens.0);
        for _ch in 0..length - expr_lens.0 as usize {
            let next_num = number_iter.next().unwrap().to_string();
            after.push_str(&next_num);
        }

        if after.is_empty() {
            println!("after is empty: {}", after);
            let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number};
            return Some(evaluate(expr).unwrap())

        } else {
            println!("after is not empty: {}", after);
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

    let expr_lens = get_expression_length(&first_char, &number_iter).unwrap();
    let mut third_number = String::new();
    for _i in 0..expr_lens.2 {
        third_number.push(number_iter.next().unwrap());
    }
    println!("third_num low priority: {}", third_number);

    if third_number.contains("(") {
        println!("Paren");
        third_number = evaluate_paren(&third_number, &number_iter).unwrap();
        while number_iter.next().unwrap().to_string() != ")" {
            number_iter.next();
        }
    }

    if is_number(&third_number).is_some() {
        let mut after = String::new();
        println!("Full Expression Length: {}", length);
        println!("Expression Length: {}", expr_lens.0);
        for _ch in 0..length - expr_lens.0 as usize { // 3 cause that is the size of an expression
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
    } else {
        return None
    }
}

fn get_expression_length(first_char: &str, number_iter: &Chars) -> Option<(i32, i32, i32)> {

    let first_number_iter = first_char.chars();
    let first_number_length = number_length(first_number_iter);
    let third_number_length = number_length(number_iter.clone());
    let expr_length = first_number_length + third_number_length + 1; //operator size is always 1

    return Some((expr_length, first_number_length, third_number_length))
}


fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let simple5: String = String::from("22*82-42/22");

    let result = expression(simple5);
    println!("result: {}", result.unwrap());

}

