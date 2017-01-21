

pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    REGISTERS_COUNT = 8, 
}

impl Register {
    pub const AL: Register = Register::EAX;
    pub const CL: Register = Register::ECX;
    pub const DL: Register = Register::EDX;
    pub const BL: Register = Register::EBX;
    pub const AH: Register = Register::EAX + 4; //本当は AL + 4と書きたいが ``not found in scope``になる
    pub const CH: Register = Register::ECX + 4;
    pub const DH: Register = Register::EDX + 4;
    pub const BH: Register = Register::EBX + 4;
}

struct Emulator<'a> {

    registers: [u32; Register::LENGTH],
    
    eflags: u32,

    memory: &'a u8,

    eip: u32,
        
}
