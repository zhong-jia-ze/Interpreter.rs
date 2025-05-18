use crate::vm::Execute;
use crate::value::Value;
use std::{io::{self,Write},process};
pub type LibFunc=fn(&mut Execute)->Value;
pub fn print(state:&mut Execute)->Value{
    for v in &state.stack{
        print!("{}",v);
    }
    Value::Nil
}
pub fn println(state:&mut Execute)->Value{
    for v in &state.stack{
        println!("{}",v);
    }
    Value::Nil
}
pub fn input(state:&mut Execute)->Value{
    print!("{}",state.stack[0]);
    io::stdout().flush().unwrap();
    let mut inp_string=String::new();
    io::stdin().read_line(&mut inp_string).unwrap();
    Value::String(inp_string)
}
pub fn sin(state:&mut Execute)->Value{
    if let Value::Float(x)=state.stack[0]{
        Value::Float(x.sin())
    }else if let Value::Int(x)=state.stack[0]{
        Value::Float((x as f64).sin())
    }else{Value::Nil}
}
pub fn cos(state:&mut Execute)->Value{
    if let Value::Float(x)=state.stack[0]{
        Value::Float(x.cos())
    }else if let Value::Int(x)=state.stack[0]{
        Value::Float((x as f64).cos())
    }else{Value::Nil}
}
pub fn exit(_state:&mut Execute)->Value{
    process::exit(0)
}