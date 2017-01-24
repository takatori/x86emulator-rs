
const REGISTERS_COUNT: usize = 8;
const MEMORY_SIZE: usize = 1024 * 1024;

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

struct Emulator {

    // 汎用レジスタ
    registers: [u32; REGISTERS_COUNT],     // 本当は[u32; Register::REGISTERS_COUNT],のようにしたいが、unresolved path in constant expressionになる

    // EFLAGSレジスタ
    eflags: u32,

    // メモリ
    memory: [u8; MEMORY_SIZE], // 生ポインタを使用する -> 配列で宣言する

    // プログラムカウンタ
    eip: u32,
        
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
        self.memory[(self.eip + index) as usize]
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

     fn set_memory8(&mut self, address: u32, value: u32) {
        self.memory[address as usize] = (value & 0xFF) as u8 // u32からu8へのキャストは頭を切り詰められる
    }

    pub fn set_memory32(&mut self, address: u32, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, value >> (i * 8))
        }
    }

    

}
