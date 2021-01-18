#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply
}

#[derive(Debug)]
pub enum Term {
    Simple(Operator, i64),
    Nested(Operator, Box<Vec<Term>>)
}

pub fn evaluate(terms: &Vec<Term>) -> i64 {
    let mut result = 0;

    for term in terms {
        result = match term {
            Term::Simple(op, n) => {
                match op {
                    Operator::Add => result + n,
                    Operator::Subtract => result - n,
                    Operator::Multiply => result * n
                }
            }
            Term::Nested(op, p) => {
                match op {
                    Operator::Add => result + evaluate(p),
                    Operator::Subtract => result - evaluate(p),
                    Operator::Multiply => result * evaluate(p)
                }
            }
        }
    }

    result
}

