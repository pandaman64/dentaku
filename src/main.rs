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
enum ParseResult<T,Iter: Iterator<Item=char>>{
    // parsed expression, remaining string
    Success(T,Box<Iter>),
    Fail
}

impl<T,Iter: Iterator<Item=char>> ParseResult<T,Iter>{
    fn successed(&self) -> bool{
        match self{
            &ParseResult::Success(_,_) => true,
            &ParseResult::Fail => false
        }
    }
}

//とりあえず一文字
fn num<Iter: Iterator<Item=char>>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("num");
    match iter.next(){
        Some('0') => ParseResult::Success(Expr::Number(0),iter),
        Some('1') => ParseResult::Success(Expr::Number(1),iter),
        Some('2') => ParseResult::Success(Expr::Number(2),iter),
        Some('3') => ParseResult::Success(Expr::Number(3),iter),
        Some('4') => ParseResult::Success(Expr::Number(4),iter),
        Some('5') => ParseResult::Success(Expr::Number(5),iter),
        Some('6') => ParseResult::Success(Expr::Number(6),iter),
        Some('7') => ParseResult::Success(Expr::Number(7),iter),
        Some('8') => ParseResult::Success(Expr::Number(8),iter),
        Some('9') => ParseResult::Success(Expr::Number(9),iter),
        _ => ParseResult::Fail
    }
}

fn primitive<Iter: Iterator<Item=char>>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("prim");
    num(iter)
}
/*
fn parseChar<Iter: Iterator<char>>(c: char,iter: &mut Iter) -> ParseResult<char,Iter>{
    match iter.next(){
        c => ParseResult::Success(c,iter),
        _ => Fail
    }
}
*/

fn multitive<Iter: Iterator<Item=char> + Clone>(mut iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    println!("mult");
    if let ParseResult::Success(lhs,iter) = primitive(iter){
        let mut clone = iter.clone();
        let op = match clone.next(){
            Some('*') => ParseResult::Success('*',clone),
            _ => ParseResult::Fail
        };
        if let ParseResult::Success(op,iter) = op{
            if let ParseResult::Success(rhs,iter) = primitive(iter){
                ParseResult::Success(Expr::Multiply(Box::new(lhs),Box::new(rhs)),iter)
            }
            else{
                ParseResult::Fail
            }     
        }
        else{
            ParseResult::Success(lhs,iter)
        }
    }
    else{
        ParseResult::Fail
    }
}

fn main() {
    let input = "1*2";
    println!("{:?}",multitive(Box::new(input.chars())));
}
