use std::process;

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
    let rm32: u32 = modrm.get_rm32(emu);
    let imm8: u32 = emu.get_sign_code8(0) as u32; // as i32; ?????
    emu.eip += 1;
    modrm.set_rm32(emu, rm32 + imm8);
}

pub fn add_rm32_r32(emu: &mut Emulator, modrm: &mut ModRM) {
    emu.eip += 1;
    modrm.parse_modrm(emu);
    let r32: u32 = modrm.get_r32(emu);
    let rm32: u32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, rm32 + r32);
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32: u32 = modrm.get_rm32(emu);
    let imm8: u32 = emu.get_sign_code8(0) as u32; // ?????
    emu.eip += 1;
    let result: u64 = rm32 as u64 - imm8 as u64;
    modrm.set_rm32(emu, result as u32); // ???? オーバーフロー対策っぽい
    emu.update_eflags_sub(rm32, imm8, result);
}

pub fn cmp_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let r32: u32  = modrm.get_r32(emu);
    let rm32: u32 = modrm.get_rm32(emu);
    let result: u64 = r32 as u64 - rm32 as u64;
    emu.update_eflags_sub(r32, rm32, result);
}

pub fn cmp_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32: u32 = modrm.get_rm32(emu);
    let imm8: u32 = emu.get_sign_code8(0) as u32 ; // as i32 ???
    emu.eip += 1;
    let result: u64 = rm32 as u64 - imm8 as u64;
    emu.update_eflags_sub(rm32, imm8, result);
}


pub fn code83(emu: &mut Emulator) {

    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    
    match modrm.opecode() {
        0 => add_rm32_imm8(emu, &modrm),
        5 => sub_rm32_imm8(emu, &modrm),
        7 => cmp_rm32_imm8(emu, &modrm),
        _ => { println!("not implemented: 83 /{}", modrm.opecode()); process::exit(0); }
    }
    
}

pub fn inc_rm32(emu: &Emulator, modrm: &ModRM) {
    let value: u32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, value + 1);
}


pub fn code_off(emu: &Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    
    match modrm.opecode {
        0 => inc_rm32(emu, &modrm);
        _ => { println!("not implemented: FF /{}", modrm.opecode()); process::exit(1); }
    }
}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 =  emu.get_code8(0) - 0xB8;
    let value: u32 = emu.get_code32(1);
    emu.registers[reg as usize] = values;
    emu.eip += 5;
}


pub fn mov_r8_rm8(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let rm8: u32 = emu.get_rm8(modrm);
    modrm.set_r8(emu, rm8);
}


pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let rm32: u32 = emu.get_rm32(modrm);
    modrm.set_r32(emu, rm32);
}


pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let r32: u32 = emu.get_r32(modrm);
    modrm.set_rm32(emu, r32);
}


pub fn mov_r8_rm8(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let value: u32 = emu.get_code32(0);
    emu.eip += 4;
    modrm.set_rm32(emu, vlaue);
}

pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let rm32: u32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, rm32);
}

pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let r32: u32 = modrm.get_r32(emu);
    modrm.set_r32(emu, r32);
}


pub fn short_jump(emu: &mut Emulator) {
    let diff: i8 = emu.get_sign_code8(1);
    emu.eip += (diff + 2);
}


