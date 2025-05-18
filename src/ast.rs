use crate::{lex::*,value::Value};
pub mod ast_type{
    use super::*;
    pub struct CallNode{pub name:String,pub args:Vec<AstNode>}
    pub struct IfNode{pub cond:AstNode,pub block:Vec<AstNode>,pub else_block:Vec<AstNode>}
    pub struct VariableNode{pub name:String,pub value:AstNode}
    pub struct BinaryOpNode{pub left:AstNode,pub op:Token,pub right:AstNode}
    pub struct UnaryOpNode{pub op:Token,pub exp:AstNode}
}
pub enum AstNode{
    Value(Value),
    Call(ast_type::CallNode),
    If(Box<ast_type::IfNode>),
    DefineVariable(Box<ast_type::VariableNode>),
    SetVariable(Box<ast_type::VariableNode>),
    UseVariable(String),
    BinaryOp(Box<ast_type::BinaryOpNode>),
    UnaryOp(Box<ast_type::UnaryOpNode>),
}
impl AstNode{
    pub fn ast_vec(lex:&mut Lex,start:Option<Token>,end:Token)->Result<Vec<AstNode>,String>{
        let mut ast_vec:Vec<AstNode>=Vec::new();
        if let Some(t)=start{
            if lex.next()!=t{return Err(format!("expected:{:?}",t))}
        }
        while lex.peek()!=end{
            ast_vec.push(Self::parse_binary_op(lex,0)?);
        }
        lex.next();
        Ok(ast_vec)
    }
    fn parse_binary_op(lex:&mut Lex,min_priority:u8)->Result<AstNode,String>{
        fn calc_priority(token:&Token)->u8{
            match token{
                Token::Plus|Token::Minus=>1,
                Token::Div|Token::Mul=>2,
                Token::Pow=>3,
                _=>0,
            }
        }
        let mut left=Self::parse_unary_op(lex)?;
        loop{
            let op=match lex.peek(){
                Token::Plus|Token::Minus|Token::Mul
                |Token::Div|Token::Eq|Token::Ne
                |Token::LesEq|Token::Less|Token::GreEq
                |Token::Greater|Token::Pow=>lex.peek(),
                _=>break Ok(left),
            };
            if calc_priority(&op)<min_priority{
                break Ok(left);
            }
            lex.next();
            let right=Self::parse_binary_op(lex,calc_priority(&op)+1)?;
            left=AstNode::BinaryOp(Box::new(ast_type::BinaryOpNode{left,op,right}));
        }
    }
    fn parse_unary_op(lex:&mut Lex)->Result<AstNode,String>{
        match lex.peek(){
            op@Token::Not=>{
                lex.next();
                Ok(AstNode::UnaryOp(Box::new(ast_type::UnaryOpNode{op,exp:Self::parse_unary_op(lex)?})))
            }
            op@Token::Minus=>{
                lex.next();
                Ok(AstNode::UnaryOp(Box::new(ast_type::UnaryOpNode{op,exp:Self::parse_unary_op(lex)?})))
            }
            _=>Self::parse_primary(lex),
        }
    }
    fn parse_primary(lex:&mut Lex)->Result<AstNode,String>{
        match lex.next(){
            Token::Value(v)=>Ok(AstNode::Value(v)),
            Token::Let=>{
                if let Token::Name(name)=lex.next(){
                    if lex.peek()==Token::Assign{
                        lex.next();
                        let value=Self::parse_binary_op(lex,0)?;
                        Ok(Self::DefineVariable(Box::new(ast_type::VariableNode{name,value})))
                    }else{
                        Ok(Self::DefineVariable(Box::new(ast_type::VariableNode{name,value:AstNode::Value(Value::Nil)})))
                    }
                }else{
                    Err("expected:variable name".to_string())
                }
            }
            Token::Name(name)=>{
                match lex.peek(){
                    Token::ParL=>{
                        lex.next();
                        let mut args:Vec<AstNode>=Vec::new();
                        loop{
                            match lex.peek(){
                                Token::ParR=>{lex.next();break Ok(Self::Call(ast_type::CallNode{name,args}))}
                                Token::Comma=>{lex.next();continue}
                                _=>args.push(Self::parse_binary_op(lex,0)?),
                            }
                        }
                    }
                    Token::Assign=>{
                        lex.next();
                        let value=Self::parse_binary_op(lex,0)?;
                        Ok(Self::SetVariable(Box::new(ast_type::VariableNode{name,value})))
                    }
                    _=>Ok(Self::UseVariable(name)),
                }
            }
            Token::If=>{
                let cond=Self::parse_binary_op(lex,0)?;
                let block=Self::ast_vec(lex,Some(Token::CurlyL),Token::CurlyR)?;
                let else_block=if lex.peek()==Token::Else{
                    lex.next();
                    Self::ast_vec(lex,Some(Token::CurlyL),Token::CurlyR)?
                }else{Vec::new()};
                Ok(AstNode::If(Box::new(ast_type::IfNode{cond,block,else_block})))
            }
            t=>Err(format!("Error Token:{:?}",t)),
        }
    }
}