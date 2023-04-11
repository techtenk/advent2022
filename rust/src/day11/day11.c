#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "day11.h"

uint64_t add(uint64_t old, int x) {
    return old + x;
}

uint64_t multiply(uint64_t old, int x) {
    return old * x;
}

uint64_t square(uint64_t old) {
    return old * old;
}

uint64_t inspect(struct Monkey *monkey, uint64_t item) {
    uint64_t temp_worry;
    if (monkey->operation == ADD) {
        temp_worry = item + monkey->operand;
    } else if (monkey->operation == MULTIPLY) {
        temp_worry = item * monkey->operand;
    } else {
        temp_worry = item * item; // SQUARE
    }
    
    return temp_worry;

    // now divide by 3 because the item wasn't broken during inspection
    // return temp_worry / 3; // integer division makes this a little too convenient to do floor() - this works because worry is always positive
}

/**
 * decide which monkey to throw to next
 * @param monkey The Monkey making the decision
 * @param item The worry level of the item
 * @returns The index of the monkey to throw to
*/
int decide(struct Monkey *monkey, uint64_t item) {
    if (item % monkey->test_divisible_by == 0) {
        return monkey->true_throw;
    }
    return monkey->false_throw;
}

void throw(struct Monkey *receiver, uint64_t item) {
    // awkwardly find the end of the items to know where to store it
    int i;
    for (i = 0; i < 36; i++) {
        if (receiver->items[i] == 0) {
            break;
        }
    }
    receiver->items[i] = item;
}

void take_turn(struct Monkey *monkeys, int curr_monkey) {
    int logging = 0;
    if (getenv("DEBUG_VERBOSE")) {
        logging = 1;
    }

    uint32_t common_divisor = monkeys->test_divisible_by;
    for (int i = 1; i < 8; i++) {
        common_divisor *= monkeys[i].test_divisible_by;
    }

    for (int item = 0; item < 36; item++) { // # of items is hardcoded to 36
        uint64_t worry_level = monkeys[curr_monkey].items[item];
        if (worry_level == 0) {
            break;
        }
        if (logging) { printf("Monkey %i inspecting item %i with worry level %lu \n", curr_monkey, item, worry_level); }

        uint64_t new = inspect(&(monkeys[curr_monkey]), monkeys[curr_monkey].items[item]);
        if (logging) { printf("Finished inspecting another item. \n"); }
        monkeys[curr_monkey].items_inspected++;

        if (logging) { printf("Item worry level: %lu \n", new); }
        if (new < 0) {
            printf("Error: overflow! \n");
            exit(-1);
        }

        int throw_to = decide(&(monkeys[curr_monkey]), new);

        if (logging) { printf("Throwing item to monkey %i \n", throw_to); }

        // before throwing, give the number a haircut by the common divisor and add back in one divisor, this will
        // keep our numbers small without changing the decisions
        uint64_t test = new % common_divisor;
        if (test < new) {
            new = test + common_divisor;
        }

        throw(&(monkeys[throw_to]), new);
        monkeys[curr_monkey].items[item] = 0;

        if (logging) { 
            printf("Monkey %i has items: [", throw_to);
            for (int i = 0; i < 10; i++) {
                printf("%lu,", monkeys[throw_to].items[i]);
            }
            printf("]\n");
        }
    }

}

// int main() {
//     printf("Welcome back to c! \n");

//     // model each monkey
//     struct Monkey *monkeys;
//     monkeys = calloc(6, sizeof(struct Monkey));

//     if (monkeys == NULL) {
//         printf("Could not allocate memory for Monkeys!");
//     }

//     int items[] = { 54, 98, 50, 94, 69, 62, 53, 85 };
//     memcpy(monkeys[0].items, items, sizeof(items));
//     monkeys[0].operation = MULTIPLY;
//     monkeys[0].operand = 13;
//     monkeys[0].test_divisible_by = 3;
//     monkeys[0].true_throw = 2;
//     monkeys[0].false_throw = 1;
//     monkeys[0].items_inspected = 0;
    
//     for (int round = 0; round < 20; round++) {
//         take_turn(monkeys, 0);
//     }


//     free(monkeys);

// }