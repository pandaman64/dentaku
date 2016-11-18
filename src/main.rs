use std::iter::Iterator;
use std::boxed::Box;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug,Clone)]
enum Expr {
    Number(u32),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
}

type ParseResult<T, Iter> = Result<(T, Parser<Iter>), ()>;

#[derive(Default,Clone,Debug)]
struct Memo {
    mchar: Option<(usize, char)>,
    mnum: Option<(usize, u32)>,
    mprim: Option<(usize, Expr)>,
    mmult: Option<(usize, Expr)>,
    madd: Option<(usize, Expr)>,
}

#[derive(Debug,Clone)]
struct Parser<Iter: Iterator<Item = (usize, char)> + Clone>(Box<Iter>,usize);

fn parse<Iter: Iterator<Item = (usize, char)> + Clone>(iter: Box<Iter>) -> ParseResult<Expr, Iter> {
    let count = iter.clone().count();
    let mut v = vec![];
    v.resize(count + 1, Memo::default());
    Parser(iter, 0).dvadditive(&mut v)
}

impl<Iter: Iterator<Item = (usize, char)> + Clone> Parser<Iter> {
    fn ctoi(c: char) -> Result<u32, ()> {
        match c {
            '0' => Ok(0),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            _ => Err(()),
        }
    }

    fn dvchar(mut self, memo: &mut Vec<Memo>) -> ParseResult<char, Iter> {
        if let Some((i, c)) = memo[self.1].mchar {
            for _ in 0..i - self.1 {
                self.0.next();
            }
            Ok((c, Parser(self.0, i)))
        } else {
            println!("char");
            if let Some((i, c)) = self.0.next() {
                memo[self.1].mchar = Some((i + 1, c));
                Ok((c, Parser(self.0, self.1 + 1)))
            } else {
                Err(())
            }
        }
    }

    fn dvnumber(self, memo: &mut Vec<Memo>) -> ParseResult<u32, Iter> {
        self.dvchar(memo).and_then(|(c, deriv)| Self::ctoi(c).map(|i| (i, deriv)))
    }

    fn dvnumber_literal_impl(self, accum: u32, memo: &mut Vec<Memo>) -> (u32, Parser<Iter>) {
        if let Ok((i, deriv)) = self.clone().dvnumber(memo) {
            deriv.dvnumber_literal_impl(accum * 10 + i, memo)
        } else {
            (accum, self)
        }
    }

    fn dvnumber_literal(mut self, memo: &mut Vec<Memo>) -> ParseResult<u32, Iter> {
        if let Some((i, num)) = memo[self.1].mnum {
            for _ in 0..i - self.1 {
                self.0.next();
            }
            Ok((num, Parser(self.0, i)))
        } else {
            println!("number");
            let start = self.1;
            self.dvnumber(memo).and_then(|(i, deriv)| {
                let ret = deriv.dvnumber_literal_impl(i, memo);
                memo[start].mnum = Some(((ret.1).1, i));
                Ok(ret)
            })
        }
    }

    fn dvprimary(mut self, memo: &mut Vec<Memo>) -> ParseResult<Expr, Iter> {
        if let Some((i, expr)) = memo[self.1].mprim.clone() {
            for _ in 0..i - self.1 {
                self.0.next();
            }
            Ok((expr, Parser(self.0, i)))
        } else {
            println!("primary");
            let start = self.1;
            match self.clone().dvchar(memo) {
                Ok(('(', deriv)) => {
                    match deriv.dvadditive(memo) {
                        Ok((expr, deriv)) => {
                            match deriv.dvchar(memo) {
                                Ok((')', deriv)) => {
                                    memo[start].mprim = Some((deriv.1, expr.clone()));
                                    Ok((expr, deriv))
                                }
                                _ => Err(()),
                            }
                        }
                        _ => Err(()),
                    }
                }
                _ => {
                    match self.dvnumber_literal(memo) {
                        Ok((i, deriv)) => {
                            memo[start].mprim = Some((deriv.1, Expr::Number(i)));
                            Ok((Expr::Number(i), deriv))
                        }
                        Err(()) => Err(()),
                    }
                }
            }
        }
    }

    fn dvmultitive(mut self, memo: &mut Vec<Memo>) -> ParseResult<Expr, Iter> {
        if let Some((i, expr)) = memo[self.1].mmult.clone() {
            for _ in 0..i - self.1 {
                self.0.next();
            }
            Ok((expr, Parser(self.0, i)))
        } else {
            println!("multitive");
            let start = self.1;
            if let Ok((lhs, deriv)) = self.dvprimary(memo) {
                match deriv.clone().dvchar(memo) {
                    Ok(('*', deriv)) => {
                        match deriv.dvmultitive(memo) {
                            Ok((rhs, deriv)) => {
                                let expr = Expr::Multiply(Box::new(lhs), Box::new(rhs));
                                memo[start].mmult = Some((deriv.1, expr.clone()));
                                Ok((expr, deriv))
                            }
                            Err(_) => Err(()),
                        }
                    }
                    Ok(('/', deriv)) => {
                        match deriv.dvmultitive(memo) {
                            Ok((rhs, deriv)) => {
                                let expr = Expr::Divide(Box::new(lhs), Box::new(rhs));
                                memo[start].mmult = Some((deriv.1, expr.clone()));
                                Ok((expr, deriv))
                            }
                            Err(_) => Err(()),
                        }
                    }
                    _ => {
                        memo[start].mmult = Some((deriv.1, lhs.clone()));
                        Ok((lhs, deriv))
                    }
                }
            } else {
                Err(())
            }
        }
    }

    fn dvadditive(mut self, memo: &mut Vec<Memo>) -> ParseResult<Expr, Iter> {
        if let Some((i, expr)) = memo[self.1].madd.clone() {
            for _ in 0..i - self.1 {
                self.0.next();
            }
            Ok((expr, Parser(self.0, i)))
        } else {
            println!("additive");
            let start = self.1;
            if let Ok((lhs, deriv)) = self.dvmultitive(memo) {
                match deriv.clone().dvchar(memo) {
                    Ok(('+', deriv)) => {
                        match deriv.dvadditive(memo) {
                            Ok((rhs, deriv)) => {
                                let expr = Expr::Add(Box::new(lhs), Box::new(rhs));
                                memo[start].madd = Some((deriv.1, expr.clone()));
                                Ok((expr, deriv))
                            }
                            Err(_) => Err(()),
                        }
                    }
                    Ok(('-', deriv)) => {
                        match deriv.dvadditive(memo) {
                            Ok((rhs, deriv)) => {
                                let expr = Expr::Subtract(Box::new(lhs), Box::new(rhs));
                                memo[start].madd = Some((deriv.1, expr.clone()));
                                Ok((expr, deriv))
                            }
                            Err(_) => Err(()),
                        }
                    }
                    _ => {
                        memo[start].madd = Some((deriv.1, lhs.clone()));
                        Ok((lhs, deriv))
                    }
                }
            } else {
                Err(())
            }
        }
    }
}

fn main() {
    match parse(Box::new("(12+34)*56-78".chars().enumerate())) {
        Ok((expr, _)) => println!("{:?}", expr),
        Err(()) => println!("parse error"),
    };
    match parse(Box::new("".chars().enumerate())) {
        Ok((expr, _)) => println!("{:?}", expr),
        Err(()) => println!("parse error"),
    };
}
