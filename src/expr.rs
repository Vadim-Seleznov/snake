use crate::scanner::Token;

pub enum LiteralValue {
    Number(f64),
    Str(String),
    True,
    False,
    Nil
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::Str(str) => str.clone(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, op: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary 
                { left, op, right } => format!("({} {} {})", op.lexeme, left.to_string(), right.to_string()), 
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme, right.to_string()),
            _ => todo!(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::*;

    #[test]
    fn handle_expr_to_string() {
        let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, 1);
        let num = Box::from(Expr::Literal { value: LiteralValue::Number(123.0) });
        let times = Token::new(TokenType::Star, "*".to_string(), None, 1);
        let right_expr = Box::from(Expr::Grouping { expression: Box::from(Expr::Literal { value: LiteralValue::Number(457.0) }) });
        let ast = Expr::Binary { left: Box::from(Expr::Unary { operator: minus_token, right: num}), op: times, right: right_expr};

        assert_eq!(ast.to_string(), "(* (- 123) (group 457))");
    }
}