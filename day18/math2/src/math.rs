use std::ops::Mul;

#[derive(Debug)]
pub enum Term {
    Number(i64),
    Add(Box<Term>, Box<Term>),
    Multiply(Box<Term>, Box<Term>),
    Parentheses(Box<Term>)
}

impl Term {
    // pub fn number(n: i64) -> Term {
    //     Term::Number(n)
    // }

    // pub fn add(a: Term, b: Term) -> Term {
    //     if let Term::Multiply(a1, a2) = a {
    //         return Term::Multiply(a1, box Term::Add(a2, box b));
    //     }

    //     if let Term::Multiply(b1, b2) = b {
    //         return Term::Multiply(box Term::Add(box a, b1), b2);
    //     }

    //     Term::Add(box a, box b)
    // }

    // pub fn multiply(a: Term, b: Term) -> Term {
    //     if let Term::Multiply(a1, a2) = a {
    //         return Term::Multiply(a1, box Term::Add(a2, box b));
    //     }

    //     if let Term::Multiply(b1, b2) = b {
    //         return Term::Multiply(box Term::Add(box a, b1), b2);
    //     }
    // }

    // pub fn parens(term: Term) -> Term {
    //     Term::Parentheses(box term)
    // }

    pub fn eval(&self) -> i64 {
        match self {
            Term::Number(n) => *n,
            Term::Add(box a, box b) => a.eval() + b.eval(),
            Term::Multiply(box a, box b) => a.eval() * b.eval(),
            Term::Parentheses(box term) => term.eval()
        }
    }
}

// pub fn evaluate(terms: &Vec<Term>) -> i64 {
//     let mut result = Vec::new();

//     // addition pass
//     for i in 0..terms.len() {
        
//     }

//     // for term in terms {
//     //     result = match term {
//     //         Term::Simple(op, n) => {
//     //             match op {
//     //                 Operator::Add => result + n,
//     //                 Operator::Multiply => result * n
//     //             }
//     //         }
//     //         Term::Nested(op, p) => {
//     //             match op {
//     //                 Operator::Add => result + evaluate(p),
//     //                 Operator::Subtract => result - evaluate(p),
//     //                 Operator::Multiply => result * evaluate(p)
//     //             }
//     //         }
//     //     }
//     // }

//     todo!()
// }

