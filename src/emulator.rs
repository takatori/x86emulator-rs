
const REGISTERS_COUNT: usize = 8;
const MEMORY_SIZE: usize = 1024 * 1024;
const CARRY_FLAG: u32    = 1;
const ZERO_FLAG: u32     = (1 << 6);
const SIGN_FLAG: u32     = (1 << 7);
const OVERFLOW_FLAG: u32 = (1 << 11);

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

    // pub const AH: Register = Register::EAX as i32 + 4; //本当は AL + 4と書きたいが ``not found in scope``になる
    // pub const CH: Register = CL + 4;
    // pub const DH: Register = DL + 4;
    // pub const BH: Register = BL + 4;
}

pub struct Emulator {

    // 汎用レジスタ
    pub registers: [u32; REGISTERS_COUNT],     // 本当は[u32; Register::REGISTERS_COUNT],のようにしたいが、unresolved path in constant expressionになる

    // EFLAGSレジスタ
    pub eflags: u32,

    // メモリ
    pub memory: [u8; MEMORY_SIZE], // 生ポインタを使用する -> 配列で宣言する

    // プログラムカウンタ
    pub eip: u32,
        
}


impl Emulator {

    pub fn new() -> Emulator {

        Emulator {
            registers: [0; REGISTERS_COUNT],
            eflags: 0,
            memory: [0; MEMORY_SIZE],
            eip: 0
        }
    }
    
    pub fn push32(&mut self, value: u32) {
        
        let address: u32 = self.get_register32(Register::ESP as u8) - 4;
        self.set_register32(Register::ESP as u8, address);
        self.set_memory32(address, value);
        
    }

    pub fn pop32(&mut self) -> u32 {

        let address: u32 = self.get_register32(Register::ESP as u8);
        let ret: u32     = self.get_memory32(address);
        self.set_register32(Register::ESP as u8, address + 4);
        ret
    }

    // memory配列の指定した番地から8ビットの値を取得する関数
    pub fn get_code8(&self, index: i32) -> u8 {
        
        self.memory[self.calc_address(self.eip, index)]
    }

    pub fn calc_address(&self, u: u32, i: i32) -> usize {

        if i < 0 { (u - i as u32) as usize } else { (u + i as u32) as usize }
    }

    pub fn get_sign_code8(&self, index: i32) -> i8 {
        self.memory[self.calc_address(self.eip, index)] as i8
    }

    
    pub fn get_code32(&self, index: i32) -> u32 {
        (0..4).fold(0, |acc, i| acc | (self.get_code8(index + i) as u32) << (8 * i))
    }

    pub fn get_sign_code32(&self, index: i32) -> i32 {
        self.get_code32(index) as i32
    }


    pub fn get_register8(&self, index: u8) -> u8 {
        if index < 4 {
            (self.registers[index as usize] & 0xff) as u8
        } else {
            ((self.registers[(index - 4) as usize] >> 8) & 0xff) as u8
        }
    }

    pub fn set_register8(&mut self, index: u8, value: u8) {
        if index < 4 {
            let r: u32 = self.registers[index as usize] & 0xffffff00;
            self.registers[index as usize] = r | value as u32;
        } else {
            let r: u32 = self.registers[(index - 4) as usize] & 0xffff00ff;
            self.registers[(index - 4) as usize] = r | ((value as u32) << 8);
        }
    }

    pub fn get_register32(&self, index: u8) -> u32 {

        return self.registers[index as usize];
    }

    pub fn set_register32(&mut self, index: u8, value: u32) {
        self.registers[index as usize] = value;
    }

    
    pub fn get_memory8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn get_memory32(&self, address: u32) -> u32 {
        (0..4).fold(0, |acc, i| acc | (self.get_memory8(address + i) as u32) << (8 * i))
    }

    pub fn set_memory8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value & 0xFF
    }

    pub fn set_memory32(&mut self, address: u32, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, (value >> (i * 8)) as u8)
        }
    }

    pub fn set_carry(&mut self, is_carry: bool) {
        if is_carry {
            self.eflags |= CARRY_FLAG;
        } else {
            self.eflags &= !CARRY_FLAG; // rustのビット反転は「!」
        }
    }

    pub fn set_zero(&mut self, is_zero: bool) {
        if is_zero {
            self.eflags |= ZERO_FLAG;
        } else {
            self.eflags &= !ZERO_FLAG;
        }
    }

    pub fn set_sign(&mut self, is_sign: bool) {
        if is_sign {
            self.eflags |= SIGN_FLAG;
        } else {
            self.eflags &= !SIGN_FLAG;
        }
    }

    pub fn set_overflow(&mut self, is_overflow: bool) {
        if is_overflow {
            self.eflags |= OVERFLOW_FLAG;
        } else {
            self.eflags &= !OVERFLOW_FLAG;
        }
    }


    pub fn is_carry(&self) -> bool {
        (self.eflags & CARRY_FLAG) != 0
    }

    pub fn is_zero(&self) -> bool {
        (self.eflags & ZERO_FLAG) != 0
    }    

    pub fn is_sign(&self) -> bool {
        (self.eflags & SIGN_FLAG) != 0
    }

    pub fn is_overflow(&self) -> bool {
        (self.eflags & OVERFLOW_FLAG) != 0
    }


    pub fn update_eflags_sub(&mut self, v1: u32, v2: u32, result: u64) {

        let sign1: u32 = v1 >> 31;           // v1の31ビット目
        let sign2: u32 = v2 >> 31;           // v2の31ビット目
        let signr: u64 = (result >> 31) & 1; // resultの31ビット目

        self.set_carry((result >> 32) != 0);
        self.set_zero(result == 0);
        self.set_sign(signr == 1);
        self.set_overflow(sign1 != sign2 && sign1 != signr as u32);
        
    }
}
