#ifndef I8080_h
#define I8080_h
#include <stdbool.h>

typedef unsigned char u8;
typedef unsigned short u16;

#define MEMORY_SIZE 65536

typedef struct i8080_core {

    u8 memory[MEMORY_SIZE];

    u8 A;
    u8 B;
    u8 C;
    u8 D;
    u8 E;
    u8 H;
    u8 L;

    u16 stack_pointer;
    u16 program_counter;

    bool sign;
    bool zero;
    bool auxiliary_carry;
    bool parity;
    bool carry;

} i8080_core;

int i8080_step(i8080_core *core);
int i8080_load_rom(i8080_core *core, const char *filename, u16 start_address);
int i8080_init(i8080_core *core);


#endif
