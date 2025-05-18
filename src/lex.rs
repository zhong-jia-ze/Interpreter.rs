use std::{iter::Peekable,str::Chars};
use crate::{value::Value,utils};
#[derive(PartialEq,Debug,Clone)]
pub enum Token{
    ParL,ParR,Assign,Comma,
    CurlyL,CurlyR,Dot,
    Minus,Plus,Mul,Div,Eq,
    Not,Ne,Less,Greater,
    LesEq,GreEq,Pow,

    Let,If,Else,

    //VarName(String),
    Name(String),
    Value(Value),
    Eos,
}
#[derive(Clone)]
pub struct Lex<'a>{
    iter:Peekable<Chars<'a>>,
}
impl<'a> Lex<'a>{
    pub fn new(string:&'a str)->Self{
        Self{iter:string.chars().peekable()}
    }
    pub fn next(&mut self)->Token{
        while let Some(c)=self.iter.peek(){
            match c{
                '('=>{self.iter.next();return Token::ParL}
                ')'=>{self.iter.next();return Token::ParR}
                '='=>{
                    self.iter.next();
                    if self.iter.peek()==Some(&'='){
                        self.iter.next();
                        return Token::Eq
                    }
                    return Token::Assign
                }
                ','=>{self.iter.next();return Token::Comma}
                '+'=>{self.iter.next();return Token::Plus}
                '{'=>{self.iter.next();return Token::CurlyL}
                '}'=>{self.iter.next();return Token::CurlyR}
                '.'=>{self.iter.next();return Token::Dot}
                '*'=>{self.iter.next();return Token::Mul}
                '^'=>{self.iter.next();return Token::Pow}
                '<'=>{
                    self.iter.next();
                    if self.iter.peek()==Some(&'='){
                        self.iter.next();
                        return Token::LesEq
                    }
                    return Token::Less
                }
                '>'=>{
                    self.iter.next();
                    if self.iter.peek()==Some(&'='){
                        self.iter.next();
                        return Token::GreEq
                    }
                    return Token::Greater
                }
                '!'=>{
                    self.iter.next();
                    if self.iter.peek()==Some(&'='){
                        self.iter.next();
                        return Token::Ne
                    }
                    return Token::Not
                }
                '/'=>{
                    self.iter.next();
                    if self.iter.peek()==Some(&'/'){
                        self.skip_line();
                        continue;
                    }
                    return Token::Div
                }
                '-'=>{
                    self.iter.next();
                    return Token::Minus
                }
                _ if c.is_ascii_digit()=>{
                    let mut number=String::new();
                    while let Some(c)=self.iter.peek(){
                        if c.is_ascii_digit()||*c=='.'{
                            number.push(*c);
                            self.iter.next();
                        }else if number.contains('.'){
                            return Token::Value(Value::Float(number.parse().expect("[ERROR]Not Float")))
                        }else{
                            return Token::Value(Value::Int(number.parse().expect("[ERROR]Not Int")))
                        }
                    }
                }
                _ if c.is_alphabetic()||*c=='_'=>{
                    let mut name=String::new();
                    while let Some(c)=self.iter.peek(){
                        if c.is_alphabetic()||c.is_ascii_digit()||*c=='_'{
                            name.push(*c);
                            self.iter.next();
                        }else{
                            return match &name as &str{
                                "nil"=>Token::Value(Value::Nil),
                                "let"=>Token::Let,
                                "if"=>Token::If,
                                "else"=>Token::Else,
                                "true"=>Token::Value(Value::Bool(1)),
                                "false"=>Token::Value(Value::Bool(0)),
                                n=>Token::Name(n.to_string()),
                            }
                        }
                    }
                }
                '"'=>{
                    self.iter.next();
                    let mut string=String::new();
                    loop{
                        if self.iter.peek()!=Some(&'"'){
                            string.push(self.iter.next().unwrap());
                        }else{
                            self.iter.next();
                            return Token::Value(Value::String(string))
                        }
                    }
                }
                '\n'|'\r'|'\t'|' '=>{
                    self.iter.next();
                    continue;
                }
                t=>utils::show_error(&format!("Unknow Char:{}",t)),
            }
        }
        Token::Eos
    }
    pub fn peek(&mut self)->Token{
        self.clone().next()
    }
    fn skip_line(&mut self){self.iter.position(|c|c=='\n');}
}