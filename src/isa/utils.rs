use crate::hardware::{LC3, Register};

use super::types::ConditionFlag;

pub fn sign_extend(x: u16, bit_count: u32) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFFu16.wrapping_shl(bit_count))
    } else {
        x
    }
}

pub fn update_flags(vm: &mut LC3, r: usize) {
    vm.registers[Register::RCond as usize] = if vm.registers[r] == 0 {
        ConditionFlag::ZRO as u16
    } else if vm.registers[r] >> 15 == 1 {
        ConditionFlag::NEG as u16
    } else {
        ConditionFlag::POS as u16
    };
}

