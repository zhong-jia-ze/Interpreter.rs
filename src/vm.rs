use crate::value::Value;
use crate::build_in;
use crate::bytecode::ByteCode;
use crate::parse::Proto;
use crate::utils::{Map,PopX};
pub struct Execute{
    pub stack:Vec<Value>,
    pub locals:Map<String,Value>,
}
impl Execute{
    pub fn new()->Self{
        let mut locals:Map<String,Value>=Map::new();
        locals.insert("print".to_string(),Value::BuildInFunc(build_in::print));
        locals.insert("println".to_string(),Value::BuildInFunc(build_in::println));
        locals.insert("input".to_string(),Value::BuildInFunc(build_in::input));
        locals.insert("sin".to_string(),Value::BuildInFunc(build_in::sin));
        locals.insert("cos".to_string(),Value::BuildInFunc(build_in::cos));
        locals.insert("exit".to_string(),Value::BuildInFunc(build_in::exit));
        Self{stack:Vec::new(),locals}
    }
    pub fn exec(&mut self,proto:Proto)->Result<(),String>{
        let mut pc:isize=0;
        while (pc as usize)<proto.bytecodes.len(){
            match &proto.bytecodes[pc as usize]{
                ByteCode::LoadValue(idx)=>self.stack.push(proto.values[*idx as usize].clone()),
                ByteCode::Call(idx,arg_num,want_ret)=>{
                    let old_stack=if *want_ret{
                        Some(self.stack.drain(..self.stack.len()-*arg_num as usize).collect())
                    }else{None};
                    let v=if let Some(Value::BuildInFunc(func))=self.locals.get((&proto.values[*idx as usize]).into()){
                        func(self)
                    }else{return Err(format!("Not found function:{}",&proto.values[*idx as usize]))};
                    if let Some(old_stack)=old_stack{
                        self.stack=old_stack;
                        self.stack.push(v);
                    }else{self.stack.clear()}
                }
                ByteCode::DefineVar(idx)=>{
                    self.locals.insert((&proto.values[*idx as usize]).into(),self.stack.popx());
                }
                ByteCode::Move(idx)=>{
                    if let Some(value)=self.locals.get((&proto.values[*idx as usize]).into()){
                        self.stack.push(value.clone());
                    }else{
                        return Err(format!("Not found var:{}",&proto.values[*idx as usize]))
                    }
                }
                ByteCode::SetVar(idx)=>{
                    self.locals.set((&proto.values[*idx as usize]).into(),self.stack.popx());
                }
                ByteCode::IfJump(jmp)=>{
                    if matches!(self.stack.popx(),Value::Int(0)|Value::Bool(0)|Value::Nil){
                        pc+=*jmp as isize;
                    }
                }
                ByteCode::Drop(num)=>{
                    self.locals.truncate(self.locals.len()-*num as usize);
                }
                ByteCode::BinaryOp(t)=>{
                    let (left,right)=(self.stack.popx(),self.stack.popx());
                    match t{
                        0=>self.stack.push((left+right)?),
                        1=>self.stack.push((left-right)?),
                        2=>self.stack.push((left*right)?),
                        3=>self.stack.push((left/right)?),
                        4=>self.stack.push(Value::Bool((left==right) as u8)),
                        5=>self.stack.push(Value::Bool((left>=right) as u8)),
                        6=>self.stack.push(Value::Bool((left>right) as u8)),
                        7=>self.stack.push(Value::Bool((left<=right) as u8)),
                        8=>self.stack.push(Value::Bool((left<right) as u8)),
                        9=>self.stack.push(left.pow(right)?),
                        _=>unreachable!(),
                    }
                }
                ByteCode::Not=>{
                    let value=self.stack.popx();
                    self.stack.push((!value)?);
                }
                ByteCode::Neg=>{
                    let value=self.stack.popx();
                    self.stack.push((-value)?);
                }
                ByteCode::Goto(jmp)=>{
                    pc+=*jmp as isize;
                }
            }
            pc+=1;
        }
        Ok(())
    }
}