use crate::{lex::*,bytecode::ByteCode};
use crate::ast::{AstNode,ast_type};
use crate::value::Value;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Proto{
    pub bytecodes:Vec<ByteCode>,
    pub values:Vec<Value>,
    value_map:HashMap<Value,usize>,
    var_number:u16,
    function_number:u16,
}
impl Proto{
    pub fn new()->Self{
        Self{
            bytecodes:Vec::new(),
            values:Vec::new(),value_map:HashMap::new(),
            var_number:0,function_number:0
        }
    }
    pub fn parse(lex:&mut Lex)->Result<Proto,String>{
        let mut proto=Self::new();
        let ast_vec=AstNode::ast_vec(lex,None,Token::Eos)?;
        for ast in ast_vec{
            Self::statment(ast,&mut proto,false);
        }
        Ok(proto)
    }
    fn load_value(proto:&mut Proto,v:Value)->u16{
        if let Some(index)=proto.value_map.get(&v){
            return (*index) as u16
        }
        proto.values.push(v.clone());
        proto.value_map.insert(v,proto.values.len()-1);
        (proto.values.len()-1) as u16
    }
    fn statment(ast:AstNode,proto:&mut Proto,evaluate:bool){
        match ast{
            AstNode::Value(v)=>{
                if evaluate{
                    let value_index=Self::load_value(proto,v);
                    proto.bytecodes.push(ByteCode::LoadValue(value_index));
                }
            }
            AstNode::Call(ast_type::CallNode{name,args})=>{
                if evaluate{Self::parse_function(name,args,true,proto)}
                else{Self::parse_function(name,args,false,proto)}
            }
            AstNode::DefineVariable(var_ast)=>{
                proto.var_number+=1;
                let ast_type::VariableNode{name,value}=*var_ast;
                let name_index=Self::load_value(proto,Value::String(name));
                Self::statment(value,proto,true);
                proto.bytecodes.push(ByteCode::DefineVar(name_index));
            }
            AstNode::SetVariable(var_ast)=>{
                let ast_type::VariableNode{name,value}=*var_ast;
                let name_index=Self::load_value(proto,Value::String(name));
                Self::statment(value,proto,true);
                proto.bytecodes.push(ByteCode::SetVar(name_index));
            }
            AstNode::UseVariable(name)=>{
                if evaluate{
                    let name_index=Self::load_value(proto,Value::String(name));
                    proto.bytecodes.push(ByteCode::Move(name_index));
                }
            }
            AstNode::If(ast_type)=>{
                let ast_type::IfNode{cond,block,else_block}=*ast_type;
                Self::statment(cond,proto,true);
                proto.bytecodes.push(ByteCode::IfJump(0));
                let ifjump_index=proto.bytecodes.len()-1;
                if else_block.is_empty(){
                    proto.bytecodes[ifjump_index]=ByteCode::IfJump(Self::block(block,proto));
                }else{
                    proto.bytecodes[ifjump_index]=ByteCode::IfJump(Self::block(block,proto)+1);
                    proto.bytecodes.push(ByteCode::Goto(0));
                    let goto_index=proto.bytecodes.len()-1;
                    proto.bytecodes[goto_index]=ByteCode::Goto(Self::block(else_block,proto) as i32);
                }
            }
            AstNode::BinaryOp(binary_op)=>{
                if evaluate{
                    let ast_type::BinaryOpNode{left,op,right}=*binary_op;
                    Self::statment(right,proto,true);
                    Self::statment(left,proto,true);
                    match op{
                        Token::Plus=>proto.bytecodes.push(ByteCode::BinaryOp(0)),
                        Token::Minus=>proto.bytecodes.push(ByteCode::BinaryOp(1)),
                        Token::Mul=>proto.bytecodes.push(ByteCode::BinaryOp(2)),
                        Token::Div=>proto.bytecodes.push(ByteCode::BinaryOp(3)),
                        Token::Eq=>proto.bytecodes.push(ByteCode::BinaryOp(4)),
                        Token::Ne=>{
                            proto.bytecodes.push(ByteCode::BinaryOp(4));
                            proto.bytecodes.push(ByteCode::Not);
                        }
                        Token::LesEq=>proto.bytecodes.push(ByteCode::BinaryOp(7)),
                        Token::Less=>proto.bytecodes.push(ByteCode::BinaryOp(8)),
                        Token::GreEq=>proto.bytecodes.push(ByteCode::BinaryOp(5)),
                        Token::Greater=>proto.bytecodes.push(ByteCode::BinaryOp(6)),
                        Token::Pow=>proto.bytecodes.push(ByteCode::BinaryOp(9)),
                        _=>unreachable!(),
                    }
                }
            }
            AstNode::UnaryOp(unary_op)=>{
                let ast_type::UnaryOpNode{op,exp}=*unary_op;
                Self::statment(exp,proto,true);
                match op{
                    Token::Not=>proto.bytecodes.push(ByteCode::Not),
                    Token::Minus=>proto.bytecodes.push(ByteCode::Neg),
                    _=>unreachable!(),
                }
            }
        }
    }
    fn parse_function(name:String,args:Vec<AstNode>,want_return:bool,proto:&mut Proto){
        let arg_num=args.len() as u8;
        let name_index=Self::load_value(proto,Value::String(name));
        for arg in args{
            Self::statment(arg,proto,true);
        }
        proto.bytecodes.push(ByteCode::Call(name_index,arg_num,want_return))
    }
    fn block(block:Vec<AstNode>,proto:&mut Proto)->u16{
        (proto.function_number,proto.var_number)=(0,0);
        let len=proto.bytecodes.len();
        for ast in block{
            Self::statment(ast,proto,false);
        }
        proto.bytecodes.push(ByteCode::Drop(proto.function_number+proto.var_number));
        (proto.bytecodes.len()-len) as u16
    }
}