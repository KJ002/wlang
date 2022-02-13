#[derive(Debug, Clone)]
enum Token {
    Value(f64),
    Function(fn(WCode) -> WCode),
    FunctionLiteral(fn(WCode) -> WCode),
    Other(String),
}

type WCode = Vec<Token>;

fn as_nums(arr: WCode) -> Vec<f64> {
    arr.iter()
        .map(|value| match value.clone() {
            Token::Value(n) => n,
            _ => 1.0,
        })
        .collect()
}

fn as_wcode(arr: Vec<f64>) -> WCode {
    arr.iter().map(|&value| Token::Value(value)).collect()
}

fn has_function(arr: &WCode) -> bool {
    for token in arr {
        match token {
            Token::Function(_) => return true,
            _ => continue,
        }
    }

    false
}

fn get_first_bracket_open(arr: &WCode) -> Option<usize> {
    for (i, token) in arr.iter().enumerate() {
        match token {
            Token::Other(value) => {
                if value == "(" {
                    return Some(i)
                } else {
                    continue
                }
            },
            _ => continue
        }
    }

    None
}

fn get_last_bracket_close(arr: &WCode) -> Option<usize> {
    let reversed = arr.iter().rev();

    for (i, token) in reversed.enumerate() {
        match token {
            Token::Other(value) => {
                if value == "(" {
                    return Some(arr.len() - i)
                } else {
                    continue
                }
            },
            _ => continue
        }
    }

    None
}

fn sum(data: WCode) -> WCode {
    let nums = as_nums(data);
    vec![Token::Value(nums.iter().sum())]
}

fn evaluate(data: WCode) -> WCode {
    let mut new_code = data.clone();

    let final_function: fn(WCode) -> WCode = match new_code.pop() {
        Some(token) => match token {
            Token::Function(func) => func,
            _ => return data,
        },
        None => panic!("No code provided"),
    };

    let first_bracket = get_first_bracket_open(&new_code);

    if first_bracket.is_some() {
        match get_last_bracket_close(&new_code) {
            Some(last_index) => unimplemented!(),
            None => panic!("Unmatched brackets")
        }
    }

    if has_function(&new_code) {
        return final_function(evaluate(new_code));
    }

    final_function(new_code)
}

fn lexer(code: &str) -> WCode {
    code.split(" ")
        .map(|x| match x.parse::<f64>() {
            Ok(n) => Token::Value(n),
            Err(_) => {
                let mut chars = x.chars();

                if x.len() > 2 && chars.nth(0).unwrap() == '`' && chars.last().unwrap() == '`' {
                    Token::FunctionLiteral(sum)
                } else if ["(", ")"].iter().any(|&y| x == y) {
                    Token::Other(x.to_string())
                } else {
                    Token::Function(sum)
                }
            }
        })
        .collect()
}

fn main() {
    println!("{:#?}", evaluate(lexer("1 2 3 3 ( 4 ) +")));
}
