#include <stdint.h>

#ifndef day11_h__
#define day11_h__

enum operation {
    ADD,
    MULTIPLY,
    SQUARE
};

struct Monkey {
    uint64_t items[36]; // only 36 items in my input, hardcode it
    enum operation operation;
    int32_t operand;
    int32_t test_divisible_by;
    int32_t true_throw;
    int32_t false_throw;
    int32_t items_inspected;
};


extern void take_turn(struct Monkey *monkeys, int curr_monkey);

#endif

