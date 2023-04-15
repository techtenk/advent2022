
#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Operation {
    ADD = 0,
    MULTIPLY = 1,
    SQUARE = 2
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Monkey {
    pub items: [cty::uint64_t; 36], // only 36 items in my input, hardcode it
    pub operation: Operation,
    pub operand: cty::int32_t,
    pub test_divisible_by: cty::int32_t,
    pub true_throw: cty::int32_t,
    pub false_throw: cty::int32_t,
    pub items_inspected: cty::int32_t
}

#[allow(unused)]
extern "C" {
    pub fn take_turn(
        monkeys: *mut Monkey,
        curr_monkey: cty::int32_t
    );
}