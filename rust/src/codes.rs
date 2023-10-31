use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Operation {
    Branch,
    Add,
    Load,
    Store,
    JumpRegister,
    And,
    LoadRegister,
    StoreRegister,
    ReturnFromInterrupt, /* unused */
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved, /* unused */
    LoadEffectiveAddress,
    Trap,
}

#[derive(FromPrimitive)]
pub enum TrapCode {
    GetChar = 0x20,
    Out = 0x21,
    PutS = 0x22,
    In = 0x23,
    PutSP = 0x24,
    Halt = 0x25,
}
