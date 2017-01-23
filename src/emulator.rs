
const REGISTERS_COUNT:usize = 8;

pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

impl Register {
    
    pub const AL: Register = Register::EAX;
    pub const CL: Register = Register::ECX;
    pub const DL: Register = Register::EDX;
    pub const BL: Register = Register::EBX;

    pub const AH: Register = Register::EAX as i32 + 4; //本当は AL + 4と書きたいが ``not found in scope``になる
    pub const CH: Register = CL + 4;
    pub const DH: Register = DL + 4;
    pub const BH: Register = BL + 4;
}

struct Emulator<'a> {

    // 汎用レジスタ
    registers: [u32; REGISTERS_COUNT],     // 本当は[u32; Register::REGISTERS_COUNT],のようにしたいが、unresolved path in constant expressionになる

    // EFLAGSレジスタ
    eflags: u32,

    // メモリ
    memory: &'a u8,

    // プログラムカウンタ
    eip: u32,
        
}


impl <'a>Emulator<'a> {

    fn push32(&self, value: u32) {

        let address:u32 =  self.get_register32(Register::ESP as usize) - 4;
        
        
    }

    fn pop32(&self) -> u32 {
        
    }

    fn get_code8(&self, index: i32) -> i32 {
        
    }


    fn get_register32(self, index: usize) -> u32 {
        return self.registers[index];
    }


    fn set_register32(&self, index: usize, value: u32) {
        self.registers[index] = value;
    }

    

}
