#include <stdlib.h>
#include <stdio.h>
#include "i8080.h"



/*
 * i8080_init - Function
 * Expects: core to be uninitilized
 * Does: Initilizes the core struct as needed before emulation start
 * Returns: 1 if all good 0 if any failure
 */
int i8080_init(i8080_core *core){
    return 0;
}

/*
 * i8080_load_rom - Function
 * Expects: i8080_core pointer to be pointed to a initliazed i8080_core struct,
 * const char pointer filename to be pointing to a valid rom file,
 * u16 start address to be a valid starting address for the program counter
 * Does: loads the given rom to the start address on the i8080 structs memory
 * Returns: 1 if all good and 0 if any failure
 */
int i8080_load_rom(i8080_core *core, const char *filename, u16 start_address){

    return 0;
}

/*
 * i8080_step - Function
 * Expects: the i8080_core struct pointer to be pointed to both an initilized and loaded with a rom,
 * Does: performs one instruction (the instruction pointed at by the PC)
 * Returns: not 0 to denote how many cycles that instructions SHOULD take on a i8080,
 * or returns 0 to denote an error occured (irrecoverable)
 */
int i8080_step(i8080_core *core){

    u8 instruction;
    u8 temp1_8;
    u8 temp2_8;
    u16 temp3_16;

    instruction = core->memory[core->program_counter];

    switch (instruction){

        // NOP
        case 0x00:
            break;

        // Load immediate (next 2 bytes) into BC no flags
        case 0x01:
            core->C = core->memory[core->program_counter + 1];
            core->B = core->memory[core->program_counter + 2];
            core->program_counter += 3;
            break;

        // Store A into memory address BC no flags
        case 0x02:
            core->memory[(core->B << 8) | core->C] = core->A;
            core->program_counter += 1;
            break;

        // Increment BC (BC = BC + 1) no flags
        case 0x03:
            temp3_16 = ((core->B << 8) | core->C) + 1;
            core->B = temp3_16 >> 8;
            core->C = temp3_16 & 0x00FF;
            core->program_counter += 1;
            break;

        // Increment B (B = B + 1) flags Z,S,P,AC
        case 0x04:
            break;

        default:
            break;

    }

    return 0;
}
