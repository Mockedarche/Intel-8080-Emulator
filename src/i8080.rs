use std::{fs, usize};

pub enum LoadRomResult {
    Ok,
    Error,
    NotFound,
}

pub enum StepInstructionResult {
    Ok,
    Error,
    NotKnownOpcode,
    NoOperation,
}

const MEMORY_SIZE: usize = 65536;

/*
 * get_twos_compliment - Helper Function
 * Expects: N/A
 * Does: Takes a u8 value and returns the twos compliment of it
 * Returns: Twos compliment of the given u8 value
 */
pub fn get_twos_compliment(value: u8) -> u8 {
    (!value).wrapping_add(1)
}

pub struct I8080Core {
    pub memory: [u8; MEMORY_SIZE],

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub program_counter: u16,
    pub stack_pointer: u16,

    pub sign: bool,
    pub zero: bool,
    pub auxiliary_carry: bool,
    pub parity: bool,
    pub carry: bool,
}

impl I8080Core {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            program_counter: 0,
            stack_pointer: 0,
            sign: false,
            zero: false,
            auxiliary_carry: false,
            parity: false,
            carry: false,
        }
    }

    pub fn i8080_load_rom(&mut self, path: &str) -> LoadRomResult {
        match fs::read(path) {
            Ok(data) => {
                if data.len() > self.memory.len() {
                    return LoadRomResult::Error;
                }
                let len = data.len();
                self.memory[..len].copy_from_slice(&data[..len]);
                LoadRomResult::Ok
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => LoadRomResult::NotFound,
            Err(_) => LoadRomResult::Error,
        }
    }
    /*
     * set_zero_flag - Function
     * Expects: self to be intialized and value to be valid data (which is to say the resulting value of
     * an opcode)
     * Does: Sets the i8080Core objects zero flag if value is zero to true else false
     */
    pub fn set_zero_flag(&mut self, value: u8) {
        self.zero = value == 0;
    }

    /*
     * set_sign_flag - Function
     * Expects: self to be intiialized and value to be valid data (which is to say the resulting value of
     * an opcode)
     * Does: Sets the i8080Core objects sign flag if value's most sig bit is 1 to true else false
     */
    pub fn set_sign_flag(&mut self, value: u8) {
        self.sign = (value >> 7) == 1;
    }

    /*
     * set_auxiliary_carry_flag - Function
     * Expects: self to be initialized and value to be valid data (which is to say the resulting value of
     * an opcode)
     * Does: Sets the i8080Core objects auxiliary carry flag if the most sig bit of the first nibble becomes the
     * first bit of the second nibble EX: 0x0F + 0x01 = 0x10
     */
    pub fn set_auxiliary_carry(&mut self, first: u8, second: u8, result: u8) {
        self.auxiliary_carry = ((first ^ second ^ result) & 0x10) != 0;
    }

    /*
     * set_parity_flag - Function
     * Expects: self to be initialized and value to be valid data (which is to say the resulting value of
     * an opcode)
     * Does: Sets the i8080Core objects parity flag if last 8 bits of value are an even amount of 1 then true
     * else false
     */
    pub fn set_parity_flag(&mut self, result: u16) {
        let low_eight = result & 0xFF;
        let mut count = 0;
        for i in 0..8 {
            if (low_eight & (1 << i)) != 0 {
                count += 1;
            }
        }
        self.parity = (count % 2) == 0;
    }

    /*
     * set_carry_flag - Function
     * Epects: self to be initialized and value to be valid data (which is to say its the correct registers value)
     * Does: Sets carry flag to the most sig bit of the value
     */
    pub fn set_carry_flag(&mut self, value: u8) {
        self.carry = (value & 0x80) != 0;
    }

    /*
     * i8080_step - Function
     * Epxects: self to be initialized
     * Does: Performs one instruction (the one pointed at by the program counter)
     * Returns:
     */
    pub fn i8080_step(&mut self) -> StepInstructionResult {
        let mut instruction: u8;
        let mut temp1_8: u8;
        let mut temp2_8: u8;
        let mut temp3_16: u16;

        instruction = self.memory[self.program_counter as usize];

        match instruction {
            0x00 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x01 => {
                self.c = self.memory[(self.program_counter as usize) + 1];
                self.b = self.memory[(self.program_counter as usize) + 2];
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x02 => {
                self.memory[(((self.b as u16) << 8) | self.c as u16) as usize] = self.a;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x03 => {
                temp3_16 = (self.b as u16) << 8 | (self.c as u16);
                temp3_16 = temp3_16.wrapping_add(1);
                self.b = (temp3_16 >> 8) as u8;
                self.c = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x04 => {
                self.b = self.b.wrapping_add(1);
                self.set_sign_flag(self.b);
                self.set_zero_flag(self.b);
                self.set_auxiliary_carry(self.b - 1, 1, self.b);
                self.set_parity_flag(self.b as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x05 => {
                self.b = self.b.wrapping_sub(1);
                self.set_sign_flag(self.b);
                self.set_zero_flag(self.b);
                self.set_auxiliary_carry(self.b + 1, get_twos_compliment(1), self.b);
                self.set_parity_flag(self.b as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x06 => {
                self.b = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x07 => {
                self.set_carry_flag(self.a);
                self.a = self.a << 1 | if self.carry { 1 } else { 0 };
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x08 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x09 => {
                let mut hl = (self.h as u16) << 8 | self.l as u16;
                let bc = (self.b as u16) << 8 | self.c as u16;
                let sum = hl.wrapping_add(bc);
                self.carry = sum < hl;
                self.h = (sum >> 8) as u8;
                self.l = sum as u8;
                return StepInstructionResult::Ok;
            }
            0x0A => {
                self.a = self.memory[(((self.b as u16) << 8) | self.c as u16) as usize];
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x0B => {
                temp3_16 = (self.b as u16) << 8 | self.c as u16;
                temp3_16 = temp3_16.wrapping_sub(1);
                self.b = (temp3_16 >> 8) as u8;
                self.c = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x0C => {
                self.c = self.c.wrapping_add(1);
                self.set_sign_flag(self.c);
                self.set_zero_flag(self.c);
                self.set_auxiliary_carry(self.c - 1, 1, self.c);
                self.set_parity_flag(self.c as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x0D => {
                self.c = self.c.wrapping_sub(1);
                self.set_sign_flag(self.c);
                self.set_zero_flag(self.c);
                self.set_auxiliary_carry(self.c + 1, get_twos_compliment(1), self.c);
                self.set_parity_flag(self.c as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x0E => {
                self.c = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x0F => {
                self.set_carry_flag(self.a);
                self.a = (self.a >> 1) | if self.carry { 0x80 } else { 0 };
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x10 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x11 => {
                self.e = self.memory[(self.program_counter as usize) + 1];
                self.d = self.memory[(self.program_counter as usize) + 2];
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x12 => {
                self.memory[(((self.d as u16) << 8) | self.e as u16) as usize] = self.a;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x13 => {
                temp3_16 = (self.d as u16) << 8 | (self.e as u16);
                temp3_16 = temp3_16.wrapping_add(1);
                self.d = (temp3_16 >> 8) as u8;
                self.e = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x14 => {
                self.d = self.d.wrapping_add(1);
                self.set_sign_flag(self.d);
                self.set_zero_flag(self.d);
                self.set_auxiliary_carry(self.d - 1, 1, self.d);
                self.set_parity_flag(self.d as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x15 => {
                self.d = self.d.wrapping_sub(1);
                self.set_sign_flag(self.d);
                self.set_zero_flag(self.d);
                self.set_auxiliary_carry(self.d + 1, get_twos_compliment(1), self.d);
                self.set_parity_flag(self.d as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x16 => {
                self.d = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x17 => {
                let temp_bool = self.carry;
                self.set_carry_flag(self.a);
                self.a = self.a << 1 | if temp_bool { 1 } else { 0 };
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x18 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x19 => {
                let mut hl = (self.h as u16) << 8 | self.l as u16;
                let de = (self.d as u16) << 8 | self.e as u16;
                let sum = hl.wrapping_add(de);
                self.carry = sum < hl;
                self.h = (sum >> 8) as u8;
                self.l = sum as u8;
                return StepInstructionResult::Ok;
            }
            0x1A => {
                self.a = self.memory[(((self.d as u16) << 8) | self.e as u16) as usize];
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x1B => {
                temp3_16 = (self.d as u16) << 8 | self.e as u16;
                temp3_16 = temp3_16.wrapping_sub(1);
                self.d = (temp3_16 >> 8) as u8;
                self.e = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x1C => {
                self.e = self.e.wrapping_add(1);
                self.set_sign_flag(self.e);
                self.set_zero_flag(self.e);
                self.set_auxiliary_carry(self.e - 1, 1, self.e);
                self.set_parity_flag(self.e as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x1D => {
                self.e = self.e.wrapping_sub(1);
                self.set_sign_flag(self.e);
                self.set_zero_flag(self.e);
                self.set_auxiliary_carry(self.e + 1, get_twos_compliment(1), self.e);
                self.set_parity_flag(self.e as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x1E => {
                self.e = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x1F => {
                let temp_bool = self.carry;
                self.set_carry_flag(self.a);
                self.a = (self.a >> 1) | if temp_bool { 0x80 } else { 0 };
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x20 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x21 => {
                self.h = self.memory[(self.program_counter as usize) + 1];
                self.l = self.memory[(self.program_counter as usize) + 2];
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x22 => {
                let addr = (self.memory[(self.program_counter as usize) + 2] as u16) << 8
                    | (self.memory[(self.program_counter as usize) + 1] as u16);
                self.memory[addr as usize] = self.l;
                self.memory[addr as usize + 1] = self.h;
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x23 => {
                temp3_16 = (self.h as u16) << 8 | (self.l as u16);
                temp3_16 = temp3_16.wrapping_add(1);
                self.h = (temp3_16 >> 8) as u8;
                self.l = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x24 => {
                self.h = self.h.wrapping_add(1);
                self.set_sign_flag(self.h);
                self.set_zero_flag(self.h);
                self.set_auxiliary_carry(self.h - 1, 1, self.h);
                self.set_parity_flag(self.h as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x25 => {
                self.h = self.h.wrapping_sub(1);
                self.set_sign_flag(self.h);
                self.set_zero_flag(self.h);
                self.set_auxiliary_carry(self.h + 1, get_twos_compliment(1), self.h);
                self.set_parity_flag(self.h as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x26 => {
                self.h = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x27 => {
                if ((self.a & 0x0F) > 9 || self.auxiliary_carry) {
                    self.a = self.a.wrapping_add(6);
                    self.set_auxiliary_carry(self.a - 0x06, 0x06, self.a);
                }
                self.set_carry_flag(self.a);
                if (((self.a & 0xF0) >> 4) > 9 || self.carry) {
                    self.a = self.a.wrapping_add(0x60);
                }
                self.set_carry_flag(self.a);
                self.set_sign_flag(self.a);
                self.set_zero_flag(self.a);
                self.set_parity_flag(self.a as u16);
                return StepInstructionResult::Ok;
            }
            0x28 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x29 => {
                let mut hl = (self.h as u16) << 8 | self.l as u16;
                let sum = hl.wrapping_add(hl);
                self.carry = sum < hl;
                self.h = (sum >> 8) as u8;
                self.l = sum as u8;
                return StepInstructionResult::Ok;
            }
            0x2A => {
                let addr = (self.memory[(self.program_counter as usize) + 2] as u16) << 8
                    | (self.memory[(self.program_counter as usize) + 1] as u16);
                self.l = self.memory[addr as usize];
                self.h = self.memory[addr as usize + 1];
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x2B => {
                temp3_16 = (self.h as u16) << 8 | self.l as u16;
                temp3_16 = temp3_16.wrapping_sub(1);
                self.h = (temp3_16 >> 8) as u8;
                self.l = (temp3_16 & 0x00FF) as u8;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x2C => {
                self.l = self.l.wrapping_add(1);
                self.set_sign_flag(self.l);
                self.set_zero_flag(self.l);
                self.set_auxiliary_carry(self.l - 1, 1, self.l);
                self.set_parity_flag(self.l as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x2D => {
                self.l = self.l.wrapping_sub(1);
                self.set_sign_flag(self.l);
                self.set_zero_flag(self.l);
                self.set_auxiliary_carry(self.l + 1, get_twos_compliment(1), self.l);
                self.set_parity_flag(self.l as u16);
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x2E => {
                self.l = self.memory[(self.program_counter as usize) + 1];
                self.program_counter = self.program_counter.wrapping_add(2);
                return StepInstructionResult::Ok;
            }
            0x2F => {
                self.a = !self.a;
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::Ok;
            }
            0x30 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                return StepInstructionResult::NoOperation;
            }
            0x31 => {
                self.stack_pointer = (self.memory[(self.program_counter as usize) + 1] as u16) << 8
                    | self.memory[(self.program_counter as usize) + 2] as u16;
                self.program_counter = self.program_counter.wrapping_add(3);
                return StepInstructionResult::Ok;
            }
            0x32..=0xFF => {
                return StepInstructionResult::Ok;
            }
        }
    }
}
