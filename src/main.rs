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
            _ => Err(())
        }
    }

    fn dvchar(mut self) -> ParseResult<char,Iter>{
        if let Some(c) = self.0.next(){
            Ok((c,self))
        }
        else{
            Err(())
        }
    }

    fn dvnumber(self) -> ParseResult<u32,Iter>{
        self.dvchar().and_then(|(c,deriv)|{
            Self::ctoi(c).map(|i| (i,deriv))
        })
    }

    fn dvnumber_literal_impl(self,accum: u32) -> ParseResult<u32,Iter>{
        if let Ok((i,deriv)) = self.clone().dvnumber(){
            deriv.dvnumber_literal_impl(accum * 10 + i)
        }
        else{
            Ok((accum,self))
        }
    }

    fn dvnumber_literal(self) -> ParseResult<u32,Iter>{
        self.dvnumber().and_then(|(i,deriv)|{
            deriv.dvnumber_literal_impl(i)
        })
    }

    fn dvprimary(self) -> ParseResult<Expr,Iter>{
        match self.clone().dvchar(){
            Ok(('(',deriv)) => match deriv.dvadditive(){
                Ok((expr,deriv)) => match deriv.dvchar(){
                    Ok((')',deriv)) => Ok((expr,deriv)),
                    _ => Err(())
                },
                _ => Err(())
            },
            _ => match self.dvnumber_literal(){
                Ok((i,deriv)) => Ok((Expr::Number(i),deriv)),
                Err(()) => Err(())
            }
        }
    }

    fn dvmultitive(self) -> ParseResult<Expr,Iter>{
        if let Ok((lhs,deriv)) = self.dvprimary(){
            match deriv.clone().dvchar(){
                Ok(('*',deriv)) => {
                    match deriv.dvmultitive(){
                        Ok((rhs,deriv)) => Ok((Expr::Multiply(Box::new(lhs),Box::new(rhs)),deriv)),
                        Err(_) => Err(())
                    }
                },
                Ok(('/',deriv)) => {
                    match deriv.dvmultitive(){
                        Ok((rhs,deriv)) => Ok((Expr::Divide(Box::new(lhs),Box::new(rhs)),deriv)),
                        Err(_) => Err(())
                    }
                },
                _ => Ok((lhs,deriv))
            }
        }
        else{
            Err(())
        }
    }

    fn dvadditive(self) -> ParseResult<Expr,Iter>{
        if let Ok((lhs,deriv)) = self.dvmultitive(){
            match deriv.clone().dvchar(){
                Ok(('+',deriv)) => {
                    match deriv.dvadditive(){
                        Ok((rhs,deriv)) => Ok((Expr::Add(Box::new(lhs),Box::new(rhs)),deriv)),
                        Err(_) => Err(())
                    }
                },
                Ok(('-',deriv)) => {
                    match deriv.dvadditive(){
                        Ok((rhs,deriv)) => Ok((Expr::Subtract(Box::new(lhs),Box::new(rhs)),deriv)),
                        Err(_) => Err(())
                    }
                },
                _ => Ok((lhs,deriv))
            }
        }
        else{
            Err(())
        }
    }
}

fn main() {
    match parse(Box::new("(12+34)*56-78".chars())){
        Ok((expr,_)) => println!("{:?}",expr),
        Err(()) => println!("parse error")
    };
    match parse(Box::new("".chars())){
        Ok((expr,_)) => println!("{:?}",expr),
        Err(()) => println!("parse error")
    };
}
