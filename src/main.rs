use std::iter::Iterator;
use std::boxed::Box;
use std::result::Result;

#[derive(Debug,Clone)]
enum Expr {
    Number(u32),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    // FIXME: typo
    Subtract(Box<Expr>, Box<Expr>),
}

type ParseResult<T,Iter> = Result<(T, Parser<Iter>), ()>;

#[derive(Debug,Clone)]
struct Parser<Iter: Iterator<Item=char> + Clone>(Box<Iter>);

fn parse<Iter: Iterator<Item=char> + Clone>(iter: Box<Iter>) -> ParseResult<Expr,Iter>{
    Parser(iter).dvadditive()
}

impl<Iter: Iterator<Item=char> + Clone> Parser<Iter>{
    fn ctoi(c: char) -> Result<u32,()>{
        match c{
            '0' => Result::Ok(0),
            '1' => Result::Ok(1),
            '2' => Result::Ok(2),
            '3' => Result::Ok(3),
            '4' => Result::Ok(4),
            '5' => Result::Ok(5),
            '6' => Result::Ok(6),
            '7' => Result::Ok(7),
            '8' => Result::Ok(8),
            '9' => Result::Ok(9),
            _ => Result::Err(())
        }
    }

    fn dvchar(mut self) -> ParseResult<char,Iter>{
        if let Some(c) = self.0.next(){
            Result::Ok((c,self))
        }
        else{
            Result::Err(())
        }
    }

    fn dvnumber(self) -> ParseResult<u32,Iter>{
        self.dvchar().and_then(|(c,deriv)|{
            Self::ctoi(c).map(|i| (i,deriv))
        })
    }

    fn dvnumber_literal_impl(self,accum: u32) -> ParseResult<u32,Iter>{
        if let Result::Ok((i,deriv)) = self.clone().dvnumber(){
            deriv.dvnumber_literal_impl(accum * 10 + i)
        }
        else{
            Result::Ok((accum,self))
        }
    }

    fn dvnumber_literal(self) -> ParseResult<u32,Iter>{
        self.dvnumber().and_then(|(i,deriv)|{
            deriv.dvnumber_literal_impl(i)
        })
    }

    fn dvprimary(self) -> ParseResult<Expr,Iter>{
        match self.clone().dvchar(){
            Result::Ok(('(',deriv)) => match deriv.dvadditive(){
                Result::Ok((expr,deriv)) => match deriv.dvchar(){
                    Result::Ok((')',deriv)) => Result::Ok((expr,deriv)),
                    _ => Result::Err(())
                },
                _ => Result::Err(())
            },
            _ => match self.dvnumber_literal(){
                Result::Ok((i,deriv)) => Result::Ok((Expr::Number(i),deriv)),
                Result::Err(()) => Result::Err(())
            }
        }
    }

    fn dvmultitive(self) -> ParseResult<Expr,Iter>{
        if let Result::Ok((lhs,deriv)) = self.dvprimary(){
            match deriv.clone().dvchar(){
                Result::Ok(('*',deriv)) => {
                    match deriv.dvmultitive(){
                        Result::Ok((rhs,deriv)) => Result::Ok((Expr::Multiply(Box::new(lhs),Box::new(rhs)),deriv)),
                        Result::Err(_) => Result::Err(())
                    }
                },
                Result::Ok(('/',deriv)) => {
                    match deriv.dvmultitive(){
                        Result::Ok((rhs,deriv)) => Result::Ok((Expr::Divide(Box::new(lhs),Box::new(rhs)),deriv)),
                        Result::Err(_) => Result::Err(())
                    }
                },
                _ => Result::Ok((lhs,deriv))
            }
        }
        else{
            Result::Err(())
        }
    }

    fn dvadditive(self) -> ParseResult<Expr,Iter>{
        if let Result::Ok((lhs,deriv)) = self.dvmultitive(){
            match deriv.clone().dvchar(){
                Result::Ok(('+',deriv)) => {
                    match deriv.dvadditive(){
                        Result::Ok((rhs,deriv)) => Result::Ok((Expr::Add(Box::new(lhs),Box::new(rhs)),deriv)),
                        Result::Err(_) => Result::Err(())
                    }
                },
                Result::Ok(('-',deriv)) => {
                    match deriv.dvadditive(){
                        Result::Ok((rhs,deriv)) => Result::Ok((Expr::Subtract(Box::new(lhs),Box::new(rhs)),deriv)),
                        Result::Err(_) => Result::Err(())
                    }
                },
                _ => Result::Ok((lhs,deriv))
            }
        }
        else{
            Result::Err(())
        }
    }
}

fn main() {
    match parse(Box::new("(12+34)*56-78".chars())){
        Result::Ok((expr,_)) => println!("{:?}",expr),
        Result::Err(()) => println!("parse error")
    };
    match parse(Box::new("".chars())){
        Result::Ok((expr,_)) => println!("{:?}",expr),
        Result::Err(()) => println!("parse error")
    };
}
