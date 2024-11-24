
struct Expression {
    lhs: String,
    opp: String,
    rhs: String, 
    priority: i8,
}

fn is_number(n: &str) -> Option<i32> {
    let result: Option<i32> = match n.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_e) => None,
    };

    return result;
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
    let first_char = next.next().unwrap().to_string();
    println!("first_char: {}", first_char);

    if is_number(&first_char).is_some() {
        let second_char = next.next().unwrap().to_string();
        println!("second_char: {}", second_char);

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
                let third_number = next.next().unwrap().to_string();
                println!("third_num high priority: {}", third_number);

                if is_number(&third_number).is_some() {
                    let mut after = String::new();
                    for _ch in 0..exp.len() - 3 {
                        let next_num = next.next().unwrap().to_string();
                        after.push_str(&next_num);
                    }
                    if after.is_empty() {
                        let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number, priority: exp_priority};
                        return Some(evaluate(expr).unwrap())
                    } else {
                        let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number, priority: exp_priority};
                        let result = format!("{}{}", evaluate(expr).unwrap(), after);
                        let final_result = expression(result);
                        return Some(final_result.unwrap())
                    }
                } else {
                    return None
                }

            } else if exp_priority == 0 {
                let third_number = next.next().unwrap().to_string();
                println!("third_num low priority: {}", third_number);
                let mut after = third_number.clone();
                for _ch in 0..exp.len() - 3 {
                    let next_num = next.next().unwrap().to_string();
                    after.push_str(&next_num);
                }
                if after.is_empty() {
                    println!("after is empty: {}", after);
                    let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number, priority: exp_priority};
                    return Some(evaluate(expr).unwrap())
                } else {
                    println!("after is not empty: {}", after);
                    let third_number = expression(after);
                    println!("third_num after recusion: {}", third_number.unwrap().to_string());
                    let expr = Expression {lhs: first_char, opp: second_char, rhs: third_number.unwrap().to_string(), priority: exp_priority};
                    return Some(evaluate(expr).unwrap())
                }

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


fn main() {
    let simple: String = String::from("1+2");
    let simple2: String = String::from("6/2");
    let simple3: String = String::from("6+2/2");
    let simple4: String = String::from("6/2+2");

    let result = expression(simple4);
    println!("result: {}", result.unwrap());

}

