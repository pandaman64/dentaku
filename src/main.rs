use std::iter::Iterator;
use std::boxed::Box;

#[derive(Debug)]
enum Expr{
    Number(u32),
    Multiply(Box<Expr>,Box<Expr>),
    Divide(Box<Expr>,Box<Expr>),
    Add(Box<Expr>,Box<Expr>),
    Subtract(Box<Expr>,Box<Expr>)
}

#[derive(Debug)]
struct ParseSuccess<T,Iter: Iterator<Item=char>>(T,Box<Iter>);

type ParseResult<T,Iter> = Result<ParseSuccess<T,Iter>,()>;

//とりあえず一文字
fn num<Iter: Iterator<Item=char>>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("num");
    match iter.next(){
        Some('0') => Result::Ok(ParseSuccess(Expr::Number(0),iter)),
        Some('1') => Result::Ok(ParseSuccess(Expr::Number(1),iter)),
        Some('2') => Result::Ok(ParseSuccess(Expr::Number(2),iter)),
        Some('3') => Result::Ok(ParseSuccess(Expr::Number(3),iter)),
        Some('4') => Result::Ok(ParseSuccess(Expr::Number(4),iter)),
        Some('5') => Result::Ok(ParseSuccess(Expr::Number(5),iter)),
        Some('6') => Result::Ok(ParseSuccess(Expr::Number(6),iter)),
        Some('7') => Result::Ok(ParseSuccess(Expr::Number(7),iter)),
        Some('8') => Result::Ok(ParseSuccess(Expr::Number(8),iter)),
        Some('9') => Result::Ok(ParseSuccess(Expr::Number(9),iter)),
        _ => Result::Err(())
    }
}

fn primitive<Iter: Iterator<Item=char>>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("prim");
    num(iter)
}
/*
fn parseChar<Iter: Iterator<char>>(c: char,iter: &mut Iter) -> ParseResult<char,Iter>{
    match iter.next(){
        c => Result::Ok(c,iter),
        _ => Fail
    }
}
*/

fn multitive<Iter: Iterator<Item=char> + Clone>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("mult");
    primitive(iter).and_then(|ParseSuccess(lhs,iter)|{
        let mut clone = iter.clone();
        match clone.next(){
            Some('*') => primitive(clone).map(|ParseSuccess(rhs,iter)|{
                ParseSuccess(Expr::Multiply(Box::new(lhs),Box::new(rhs)),iter)
            }),
            Some(_) => Result::Err(()),
            _ => Result::Ok(ParseSuccess(lhs,iter))
        }
    })
}

fn main() {
    let input = "1";
    println!("{:?}",multitive(Box::new(input.chars())));
    println!("{:?}",multitive(Box::new("1*2".chars())));
}
