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

pub fn inc_rm32(emu: &mut Emulator, modrm: &ModRM) {
    let value: u32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, value + 1);
}


pub fn code_off(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    
    match modrm.opecode() {
        0 => inc_rm32(emu, &modrm),
        _ => { println!("not implemented: FF /{}", modrm.opecode()); process::exit(1); }
    }
}

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: u8 =  emu.get_code8(0) - 0xB8;
    let value: u32 = emu.get_code32(1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}


pub fn mov_r8_rm8(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let rm8: u8 = modrm.get_rm8(emu);
    modrm.set_r8(emu, rm8);
}


pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let rm32: u32 = modrm.get_rm32(emu);
    modrm.set_r32(emu, rm32);
}


pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let r32: u32 = modrm.get_r32(emu);
    modrm.set_rm32(emu, r32);
}

pub fn short_jump(emu: &mut Emulator) {
    let diff: i8 = emu.get_sign_code8(1);
    emu.eip += (diff + 2) as u32;
}

pub fn near_jump(emu: &mut Emulator) {
    let diff: i32 = emu.get_sign_code32(1);
    emu.eip += (diff + 5) as u32;
}


pub fn jl(emu: &mut Emulator) {
    let diff: i8 = if is_sign(emu) != is_overflow(emu) { emu.get_sign_code8(1) } else { 0 };
    emu.eip += (diff + 2);
}

pub fn jle(emu: &mut Emulator) {
    let diff: i8 = if is_zero(emu) || is_sign(emu) != is_overflow(emu) { emu.get_sign_code8(1) } else {0};
    emu.eip += (diff + 2);
}


pub fn swi(emu: &mut Emulator) {

    let int_index: u8 = emu.get_code8(1);
    emu.eip += 2;

    match int_index {
        0x10 => bios_video(emu); // todo
        _ => println!("unknown interrupt: 0x{}", int_index); // todo
    }
}


pub fn in_al_dx(emu: &Emulator) {
    let address: u16 = emu.get_register32(Registers::EDX) & 0xffff;
    let value: u8 = io_in8(address);
    emu.set_register8(AL, value);
    emu.eip += 1;
}


pub fn out_dx_al(emu: &mut Emulator) {
    let address: u16 = emu.get_register32(EDX) & 0xffff;
    let value: u8 = emu.get_register8(AL);
    io_out8(address, value);
    emu.eip += 1;

}


pub fn mov_r8_imm8(emu: &mut Emulator) {
    let reg: u8 = emu.get_code8(0) - 0xB0;
    emu.set_register8(reg, emu.get_code8(1));
    emu.eip += 2;
}


pub fn mov_rm8_r8(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm: ModRM = ModRM::new();
    modrm.parse_modrm(emu);
    let r8: u32 = emu.get_r8(&modrm);
    emu.set_rm8(&modrm, r8);
}

pub fn cmp_al_imm8(emu: &Emulator) {
    let value: u8 = emu.get_code8(1);
    let al: u8 = emu.get_register32(AL);
    let result: u64 = al - value;
    emu.update_eflags_sub(al, value, result);
    emu.eip += 2;
}

pub fn cmp_eax_imm32(emu: &mut Emulator) {
    let value: u32 = emu.get_code32(1);
    let eax: u32 = emu.get_register32(EAX);
    let result: u64 = eax as u64 - value as u64;
    emu.update_efalg_sub(eax, value, result);
    emu.eip += 5;
}

pub fn inc_r32(emu: &mut Emulator) {
    let reg: u8 = emu.get_code8(0) - 0x40;
    emu.set_register32(reg, emu.get_register32(reg) + 1);
    emu.eip += 1;
}
    
