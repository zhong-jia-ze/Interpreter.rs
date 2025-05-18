//索引和数量(除参数数量)使用u16
//Goto跳跃的index使用i32
//参数数量和类别使用u8
#[derive(Debug)]
pub enum ByteCode{
    LoadValue(u16),
    Call(u16,u8,bool),
    DefineVar(u16),
    Move(u16),
    SetVar(u16),
    IfJump(u16),
    Drop(u16),
    BinaryOp(u8),
    Not,Neg,
    Goto(i32),
}