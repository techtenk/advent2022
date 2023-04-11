
mod day11_bindings;

include!("day11_bindings.rs");

fn get_monkeys() -> [Monkey; 8] {
    // don't really care about parsing this one, I've learned that bit
    let monkey0 = Monkey {
        items: [54, 98, 50, 94, 69, 62, 53, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::MULTIPLY,
        operand: 13,
        test_divisible_by: 3,
        true_throw: 2,
        false_throw: 1,
        items_inspected: 0
    };

    let monkey1 = Monkey {
        items: [71, 55, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::ADD,
        operand: 2,
        test_divisible_by: 13,
        true_throw: 7,
        false_throw: 2,
        items_inspected: 0
    };
    
    let monkey2 = Monkey {
        items: [77, 73, 86, 72, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::ADD,
        operand: 8,
        test_divisible_by: 19,
        true_throw: 4,
        false_throw: 7,
        items_inspected: 0
    };

    let monkey3 = Monkey {
        items: [97, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::ADD,
        operand: 1,
        test_divisible_by: 17,
        true_throw: 6,
        false_throw: 5,
        items_inspected: 0
    };

    let monkey4 = Monkey {
        items: [78, 97, 51, 85, 66, 63, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::MULTIPLY,
        operand: 17,
        test_divisible_by: 5,
        true_throw: 6,
        false_throw: 3,
        items_inspected: 0
    };

    let monkey5 = Monkey {
        items: [88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::ADD,
        operand: 3,
        test_divisible_by: 7,
        true_throw: 1,
        false_throw: 0,
        items_inspected: 0
    };

    let monkey6 = Monkey {
        items: [87, 57, 63, 86, 87, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::SQUARE,
        operand: 0,
        test_divisible_by: 11,
        true_throw: 5,
        false_throw: 0,
        items_inspected: 0
    };

    let monkey7 = Monkey {
        items: [73, 59, 82, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        operation: Operation::ADD,
        operand: 6,
        test_divisible_by: 2,
        true_throw: 4,
        false_throw: 3,
        items_inspected: 0
    };
    [monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7]
}

fn run(number_rounds: i32) {
    let mut monkeys = get_monkeys();
    for _ in 0..number_rounds {
        for i in 0..8 {
            if std::env::var("DEBUG_VERBOSE").is_ok() { println!(""); }
            unsafe {
                take_turn(monkeys.as_mut_ptr(), i as i32);
            }
        }
    }

    for (i, monkey) in monkeys.into_iter().enumerate() {
        println!("Monkey {} inspected {} items", i, monkey.items_inspected);
    }
}

pub fn run_part1() {
    // run(20);
    
}

pub fn run_part2() {
    run(10000);
}