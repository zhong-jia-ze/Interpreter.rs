use std::{collections::HashMap,hash::Hash,process};
pub fn show_error(err:&str)->!{
    println!("[Error]{}",err);
    process::exit(0)
}
pub fn _show_waring(warn:&str){
    println!("[WARING]{}",warn);
}

pub struct Map<U:Eq+Hash,T>{
    vec:Vec<T>,
    index_map:HashMap<U,usize>,
}
impl<U:Eq+Hash,T> Map<U,T>{
    pub fn new()->Self{
        Self{vec:Vec::new(),index_map:HashMap::new()}
    }
    pub fn insert(&mut self,k:U,v:T){
        self.vec.push(v);
        self.index_map.insert(k,self.len()-1);
    }
    pub fn get(&self,k:U)->Option<&T>{
        self.index_map.get(&k).and_then(|&idx|self.vec.get(idx))
    }
    pub fn truncate(&mut self,len:usize){
        self.vec.truncate(len);
        self.index_map.retain(|_,index|*index<len);
    }
    pub fn len(&self)->usize{self.vec.len()}
    pub fn set(&mut self,name:U,value:T){
        self.vec[self.index_map[&name]]=value;
    }
}

pub trait PopX<T>{
    fn popx(&mut self)->T;
}
impl<T> PopX<T> for Vec<T>{
    fn popx(&mut self)->T{
        unsafe{self.pop().unwrap_unchecked()}
    }
}