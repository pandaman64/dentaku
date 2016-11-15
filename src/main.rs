use std::iter::Iterator;
use std::boxed::Box;

#[derive(Debug)]
enum Expr {
    Number(u32),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
struct ParseSuccess<T, Iter: Iterator<Item = char>>(T, Box<Iter>);

type ParseResult<T, Iter> = Result<ParseSuccess<T, Iter>, ()>;

fn num<Iter: Iterator<Item = char>>(mut iter: Box<Iter>) -> ParseResult<u32, Iter> {
    match iter.next() {
        Some('0') => Result::Ok(ParseSuccess(0, iter)),
        Some('1') => Result::Ok(ParseSuccess(1, iter)),
        Some('2') => Result::Ok(ParseSuccess(2, iter)),
        Some('3') => Result::Ok(ParseSuccess(3, iter)),
        Some('4') => Result::Ok(ParseSuccess(4, iter)),
        Some('5') => Result::Ok(ParseSuccess(5, iter)),
        Some('6') => Result::Ok(ParseSuccess(6, iter)),
        Some('7') => Result::Ok(ParseSuccess(7, iter)),
        Some('8') => Result::Ok(ParseSuccess(8, iter)),
        Some('9') => Result::Ok(ParseSuccess(9, iter)),
        _ => Result::Err(()),
    }
}

fn backtrack<Iter: Iterator<Item = char> + Clone,
             T,
             Parser: FnOnce(Box<Iter>) -> ParseResult<T, Iter>,
             U,
             Success: FnOnce(Box<Iter>, T) -> ParseResult<U, Iter>,
             Fail: FnOnce(Box<Iter>) -> Success::Output>
    (iter: Box<Iter>,
     parser: Parser,
     success: Success,
     fail: Fail)
     -> Success::Output {
    if let Result::Ok(ParseSuccess(v, iter)) = parser(iter.clone()) {
        success(iter, v)
    } else {
        fail(iter)
    }
}

fn num_literal_impl<Iter: Iterator<Item = char> + Clone>(iter: Box<Iter>,
                                                         accum: u32)
                                                         -> ParseResult<u32, Iter> {
    backtrack(iter,
              num,
              |iter, i| num_literal_impl(iter, accum * 10 + i),
              |iter| Result::Ok(ParseSuccess(accum, iter)))
}

fn num_literal<Iter: Iterator<Item = char> + Clone>(iter: Box<Iter>) -> ParseResult<u32, Iter> {
    num(iter).and_then(|ParseSuccess(i, iter)| num_literal_impl(iter, i))
}

fn parse_char<Iter: Iterator<Item = char>>(c: char,
                                           mut iter: Box<Iter>)
                                           -> ParseResult<char, Iter> {
    match iter.next() {
        Some(d) if c == d => Result::Ok(ParseSuccess(c, iter)),
        _ => Result::Err(()),
    }
}

fn primitive<Iter: Iterator<Item = char> + Clone>(iter: Box<Iter>) -> ParseResult<Expr, Iter> {
    backtrack(iter,
              |iter| parse_char('(', iter),
              |iter, _| {
                  additive(iter).and_then(|ParseSuccess(expr, iter)| {
                      parse_char(')', iter).map(|ParseSuccess(_, iter)| ParseSuccess(expr, iter))
                  })
              },
              |iter| {
                  num_literal(iter).map(|ParseSuccess(i, iter)| ParseSuccess(Expr::Number(i), iter))
              })
}

fn multitive<Iter: Iterator<Item = char> + Clone>(iter: Box<Iter>) -> ParseResult<Expr, Iter> {
    primitive(iter).and_then(|ParseSuccess(lhs, iter)| {
        let mut clone = iter.clone();
        match clone.next() {
            Some('*') => {
                multitive(clone).map(|ParseSuccess(rhs, iter)| {
                    ParseSuccess(Expr::Multiply(Box::new(lhs), Box::new(rhs)), iter)
                })
            }
            Some('/') => {
                multitive(clone).map(|ParseSuccess(rhs, iter)| {
                    ParseSuccess(Expr::Divide(Box::new(lhs), Box::new(rhs)), iter)
                })
            }
            _ => Result::Ok(ParseSuccess(lhs, iter)),
        }
    })
}

fn additive<Iter: Iterator<Item = char> + Clone>(iter: Box<Iter>) -> ParseResult<Expr, Iter> {
    multitive(iter).and_then(|ParseSuccess(lhs, iter)| {
        let mut clone = iter.clone();
        match clone.next() {
            Some('+') => {
                additive(clone).map(|ParseSuccess(rhs, iter)| {
                    ParseSuccess(Expr::Add(Box::new(lhs), Box::new(rhs)), iter)
                })
            }
            Some('-') => {
                additive(clone).map(|ParseSuccess(rhs, iter)| {
                    ParseSuccess(Expr::Subtract(Box::new(lhs), Box::new(rhs)), iter)
                })
            }
            _ => Result::Ok(ParseSuccess(lhs, iter)),
        }
    })
}

fn main() {
    println!("{:?}", additive(Box::new("1".chars())));
    println!("{:?}", additive(Box::new("1*2".chars())));
    println!("{:?}", additive(Box::new("1+2*3-4".chars())));
    println!("{:?}", additive(Box::new("12*34+56/78".chars())));
    println!("{:?}", additive(Box::new("1+(2-3)*4+5".chars())));
}
