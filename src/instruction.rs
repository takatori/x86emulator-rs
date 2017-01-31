
use emulator::Emulator;
use emulator::Register;
use modrm::ModRM;


pub fn push_r32(emu: &mut Emulator) {
    let reg: u8 = emu.get_code8(0) - 0x50;
    let value: u32 = emu.get_register32(reg);
    emu.push32(value);
    emu.eip += 1;
}

pub fn push_imm32(emu: &mut Emulator) {
    let value: u32 = emu.get_code32(1);
    emu.push32(value);
    emu.eip += 1;
}

pub fn push_imm8(emu: &mut Emulator) {
    let value: u8 = emu.get_code8(1);
    emu.push32(value as u32);
    emu.eip += 2;
}

pub fn pop_r32(emu: &mut Emulator) {
    let reg: u8 = emu.get_code8(0) - 0x58;
    let value: u32 = emu.pop32();
    emu.set_register32(reg, value);
    emu.eip += 1;
}

pub fn call_rel32(emu: &mut Emulator) {
    let diff: i32 = emu.get_sign_code32(1);
    let eip: u32 = emu.eip;
    emu.push32(eip + 5);
    emu.eip += (diff as u32 + 5); // TODO: マイナス時の処理
}


pub fn leave(emu: &mut Emulator) {
    
    let ebp: u32 = emu.get_register32(Register::EBP as u8);
    let value: u32 = emu.pop32();
    emu.set_register32(Register::ESP as u8, ebp);
    emu.set_register32(Register::EBP as u8, value);
    emu.eip += 1;
}


pub fn ret(emu: &mut Emulator) {
    emu.eip += emu.pop32();
}


pub fn add_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32: u32 = emu.get_rm32(modrm);
    let imm8: u32 = emu.get_sign_code8(0) as i32;
}
