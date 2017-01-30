
use std::process;

use emulator::Emulator;

pub struct ModRM {
    _mod: u8,
    reg:  u8,
    rm:   u8,
    sib:  u8,
    disp: u32,
    
}

impl ModRM {

    pub fn new() -> ModRM {

        ModRM {
            _mod: 0,
            reg: 0,
            rm: 0,
            sib: 0,
            disp: 0
        }
    }

    pub fn opecode(&self) -> u8 {
        self.reg
    }

    pub fn reg_index(&self) -> u8 {
        self.reg
    }

    pub fn disp8(&self) -> i8 {
        self.disp as i8
    }

    pub fn disp32(&self) -> u32 {
        self.disp
    }
    

    pub fn parse_modrm(&mut self, emu: &mut Emulator) {

        let code: u8 = emu.get_code8(0);
        
        self._mod = (code & 0xC0) >> 6;
        self.reg  = (code & 0x38) >> 3;
        self.rm   = (code & 0x07);

        emu.eip += 1;

        if self._mod != 3 && self.rm == 4 {
            self.sib = emu.get_code8(0);
            emu.eip += 1;
        }
        
        if (self._mod == 0 && self.rm == 5) || self._mod == 2 {
            self.disp = emu.get_sign_code32(0) as u32;
        } else if self._mod == 1 {
            self.disp = emu.get_sign_code8(0) as u32;
            emu.eip += 1;
        }
    }

    pub fn calc_memory_address(&self, emu: &mut Emulator) -> u32 {
        
        if self._mod == 0 {
            if self.rm == 4 {
                println!("not mplemented ModRM mod = 0, rm = 4");
                process::exit(0);
            } else if self.rm == 5 {
                self.disp32()
            } else {
                emu.get_register32(self.rm)
            }
        } else if self._mod == 1 {
            if self.rm == 4 {
                println!("not implemented ModRM mod = 1, rm = 4");
                process::exit(0);                
            } else {
                emu.get_register32(self.rm) + self.disp8() as u32 // TODO: 引き算できるようにする
            }
        } else if self._mod == 2 {
            if self.rm == 4 {
                println!("not implemented ModRM mod = 2, rm = 4");
                process::exit(0);                                
            } else {
                emu.get_register32(self.rm) + self.disp32()
            }
        } else {
            println!("not implemented ModRM mod = 3");
            process::exit(0);                                
        }
    }

    pub fn get_rm8(&self, mut emu: &mut Emulator) -> u8 {
        if self._mod == 3 {
            emu.get_register8(self.rm)
        } else {
            let address: u32 = self.calc_memory_address(emu);
            emu.get_memory8(address)
        }
    }

    pub fn set_rm8(&self, emu: &mut Emulator, value: u8) {
        if self._mod == 3 {
            emu.set_register8(self.rm, value);
        } else {
            let address: u32 = self.calc_memory_address(emu);
            emu.set_memory8(address, value);
        }
    }

    pub fn get_rm32(&self, emu: &mut Emulator) -> u32 {
        if self._mod == 3 {
            emu.get_register32(self.rm)
        } else {
            let address: u32 = self.calc_memory_address(emu);
            emu.get_memory32(address)
        }
    }
    
    pub fn set_rm32(&self, emu: &mut Emulator, value: u32) {
        if self._mod == 3 {
            emu.set_register32(self.rm, value)
        } else {
            let address: u32 = self.calc_memory_address(emu);
            emu.set_memory32(address, value);
        }
    }

    pub fn get_r8(&self, emu: &Emulator) -> u8 {
        emu.get_register8(self.reg)
    }

    pub fn set_r8(&self, emu: &mut Emulator, value: u8) {
        emu.set_register8(self.reg, value);
    }

    pub fn get_r32(&self, emu: &Emulator) -> u32 {
        emu.get_register32(self.reg)
    }
    
    pub fn set_r32(&self, emu: &mut Emulator, value: u32) {
        emu.set_register32(self.reg, value);
    }
}
