use std::{fmt,ops,cmp::Ordering};
use crate::build_in::LibFunc;
use std::hash::{Hash,Hasher};
#[derive(PartialEq,Clone,Debug)]
pub enum Value{
    Int(i128),
    String(String),
    Nil,
    Float(f64),
    Bool(u8),
    BuildInFunc(LibFunc),
}
impl Eq for Value{}
impl fmt::Display for Value{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            Value::Int(x)=>write!(f,"{}",x),
            Value::String(x)=>write!(f,"{}",x),
            Value::Nil=>write!(f,"nil"),
            Value::Float(x)=>write!(f,"{}",x),
            Value::Bool(0)=>write!(f,"false"),
            Value::Bool(1)=>write!(f,"true"),
            Value::Bool(x)=>write!(f,"{}",x),
            Value::BuildInFunc(_)=>write!(f,"<Bulid-In-Function>"),
        }
    }
}
impl Value{
    pub fn pow(self,rhs:Self)->Result<Self,String>{
        if let (Value::Int(x),Value::Int(y))=(self,rhs){
            Ok(Value::Int(x.pow(y as u32)))
        }else{Err("Two value cannot pow".to_string())}
    }
}
impl ops::Add for Value{
    type Output=Result<Self,String>;
    fn add(self,rhs:Self)->Self::Output{
        match (self,rhs){
            (Value::Nil,Value::Nil)=>Ok(Value::Nil),
            (Value::Bool(x),Value::Bool(y))=>Ok(Value::Bool(x+y)),
            (Value::Int(a),Value::Int(b))=>Ok(Value::Int(a+b)),
            (Value::Float(a),Value::Float(b))=>Ok(Value::Float(a+b)),
            (Value::String(a),Value::String(b))=>Ok(Value::String(a+&b)),
            _=>Err("Two value cannot add".to_string()),
        }
    }
}
impl ops::Sub for Value{
    type Output=Result<Self,String>;
    fn sub(self,rhs:Self)->Self::Output{
        match (self,rhs){
            (Value::Nil,Value::Nil)=>Ok(Value::Nil),
            (Value::Int(a),Value::Int(b))=>Ok(Value::Int(a-b)),
            (Value::Float(a),Value::Float(b))=>Ok(Value::Float(a-b)),
            (Value::Bool(x),Value::Bool(y))=>Ok(Value::Bool(x-y)),
            _=>Err("Two value cannot sub".to_string()),
        }
    }
}
impl ops::Mul for Value{
    type Output=Result<Self,String>;
    fn mul(self,rhs:Self)->Self::Output{
        match (self,rhs){
            (Value::Nil,Value::Nil)=>Ok(Value::Nil),
            (Value::Int(a),Value::Int(b))=>Ok(Value::Int(a*b)),
            (Value::Float(a),Value::Float(b))=>Ok(Value::Float(a*b)),
            (Value::Bool(x),Value::Bool(y))=>Ok(Value::Bool(x*y)),
            _=>Err("Two value cannot mul".to_string()),
        }
    }
}
impl ops::Div for Value{
    type Output=Result<Self,String>;
    fn div(self,rhs:Self)->Self::Output{
        match (self,rhs){
            (Value::Nil,Value::Nil)=>Ok(Value::Nil),
            (Value::Int(a),Value::Int(b))=>Ok(Value::Int(a/b)),
            (Value::Float(a),Value::Float(b))=>Ok(Value::Float(a/b)),
            (Value::Bool(x),Value::Bool(y))=>Ok(Value::Bool(x/y)),
            _=>Err("Two value cannot div".to_string()),
        }
    }
}
impl ops::Not for Value{
    type Output=Result<Self,String>;
    fn not(self)->Self::Output{
        match self{
            Value::Bool(0)|Value::Nil=>Ok(Value::Bool(1)),
            Value::Bool(1)=>Ok(Value::Bool(0)),
            Value::Int(0)=>Ok(Value::Int(1)),
            Value::Int(1)=>Ok(Value::Int(0)),
            _=>Err("This value cannot reverse".to_string()),
        }
    }
}
impl PartialOrd for Value{
    fn partial_cmp(&self,other:&Self)->Option<Ordering>{
        match (self,other){
            (Value::Int(x),Value::Int(y))=>Some(x.cmp(y)),
            (Value::Float(x),Value::Float(y))=>{
                if *x<*y{Some(Ordering::Less)}
                else if *x>*y{Some(Ordering::Greater)}
                else{Some(Ordering::Equal)}
            }
            _=>None,
        }
    }
}
impl From<&Value> for String{
    fn from(v:&Value)->Self{
        if let Value::String(s)=v{s.to_string()}else{unreachable!()}
    }
}
impl Hash for Value{
    fn hash<H:Hasher>(&self,state:&mut H){
        match self{
            Value::Int(x)=>{
                0u8.hash(state);
                x.hash(state);
            }
            Value::Bool(x)=>{
                1u8.hash(state);
                x.hash(state);
            }
            Value::Float(x)=>{
                2u8.hash(state);
                let bit=if x.is_nan(){0x7FF8000000000000}else{x.to_bits()};
                bit.hash(state);
            }
            Value::String(s)=>{
                3u8.hash(state);
                s.hash(state);
            }
            Value::Nil=>4u8.hash(state),
            _=>(),
        }
    }
}
impl ops::Neg for Value{
    type Output=Result<Value,String>;
    fn neg(self)->Self::Output{
        match self{
            Value::Int(x)=>Ok(Value::Int(-x)),
            Value::Float(x)=>Ok(Value::Float(-x)),
            _=>Err("This value cannot neg".to_string()),
        }
    }
}