use regex::Regex;

fn operate(reg: Regex, mut expr: String, op: &str) -> String {
    if op.is_empty() {
        return "".to_string();
    }
    loop {
        let caps = reg.captures(&expr);
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let (cap_expr, left_value, right_value): (&str, i32, i32) = (
            caps.get(0).unwrap().as_str(),
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        );
        let result = match op {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };
        expr = expr.replace(cap_expr, &result.to_string());
    }
    expr
}

fn main() {
    let add_regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let sub_regex = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let mul_regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let div_regex = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    let mut expr = String::new();

    println!("Expr: ");
    std::io::stdin().read_line(&mut expr).unwrap();

    // Operations
    expr = operate(mul_regex, expr, "*");
    expr = operate(div_regex, expr, "/");
    expr = operate(add_regex, expr, "+");
    expr = operate(sub_regex, expr, "-");

    println!("\n= {expr}");
}
