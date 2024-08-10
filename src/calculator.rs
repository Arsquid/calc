use std::error::Error;

pub fn run_calc(args: &[String]) -> Result<(), Box<dyn Error>> {
    let (operation, num1, num2) = parse_args(args)?;
    let num1: f64 = num1.parse()?;
    let num2: f64 = num2.parse()?;

    let result = calculate(&operation, num1, num2)?;

    println!("Calculating...");
    println!(
        "{} {} {} = {}",
        num1,
        operation_to_symbol(&operation),
        num2,
        result
    );

    Ok(())
}

fn parse_args(args: &[String]) -> Result<(String, String, String), Box<dyn Error>> {
    if args.len() != 4 {
        return Err(format!(
            "This is a calculator program. Usage: {} <add|sub|mul|div> <num1> <num2>",
            args[0]
        )
        .into());
    }
    Ok((args[1].clone(), args[2].clone(), args[3].clone()))
}

fn calculate(operation: &str, num1: f64, num2: f64) -> Result<f64, Box<dyn Error>> {
    match operation {
        "add" => Ok(num1 + num2),
        "sub" => Ok(num1 - num2),
        "mul" => Ok(num1 * num2),
        "div" => {
            if num2 == 0.0 {
                Err("Error: Division by zero is not allowed.".into())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!(
            "Unknown operation: {}. Use 'add', 'sub', 'mul' or 'div'.",
            operation
        )
        .into()),
    }
}

fn operation_to_symbol(operation: &str) -> &str {
    match operation {
        "add" => "+",
        "sub" => "-",
        "mul" => "*",
        "div" => "/",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let args = vec![
            "calc".to_string(),
            "add".to_string(),
            "2.0".to_string(),
            "3.0".to_string(),
        ];
        assert!(run_calc(&args).is_ok());
    }

    #[test]
    fn test_subtraction() {
        let args = vec![
            "calc".to_string(),
            "sub".to_string(),
            "5.0".to_string(),
            "3.0".to_string(),
        ];
        assert!(run_calc(&args).is_ok());
    }

    #[test]
    fn test_multiplication() {
        let args = vec![
            "calc".to_string(),
            "mul".to_string(),
            "4.0".to_string(),
            "3.0".to_string(),
        ];
        assert!(run_calc(&args).is_ok());
    }

    #[test]
    fn test_division() {
        let args = vec![
            "calc".to_string(),
            "div".to_string(),
            "10.0".to_string(),
            "2.0".to_string(),
        ];
        assert!(run_calc(&args).is_ok());
    }

    #[test]
    fn test_division_by_zero() {
        let args = vec![
            "calc".to_string(),
            "div".to_string(),
            "10.0".to_string(),
            "0.0".to_string(),
        ];
        assert!(run_calc(&args).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let args = vec![
            "calc".to_string(),
            "mod".to_string(),
            "10.0".to_string(),
            "2.0".to_string(),
        ];
        assert!(run_calc(&args).is_err());
    }

    #[test]
    fn test_invalid_number() {
        let args = vec![
            "calc".to_string(),
            "add".to_string(),
            "abc".to_string(),
            "2.0".to_string(),
        ];
        assert!(run_calc(&args).is_err());
    }

    #[test]
    fn test_invalid_argument_count() {
        let args = vec!["calc".to_string(), "add".to_string(), "2.0".to_string()];
        assert!(run_calc(&args).is_err());
    }
}
