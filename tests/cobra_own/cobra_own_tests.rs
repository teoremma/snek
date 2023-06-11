use crate::*;

// Your tests go here!
success_tests! {
    {
        name: false_val,
        file: "cobra_own/false_val.snek",
        expected: "false",
    },
    {
        name: input_compare_1,
        file: "cobra_own/input_compare.snek",
        input: "2",
        expected: "false",
    },
    {
        name: input_compare_2,
        file: "cobra_own/input_compare.snek",
        input: "10",
        expected: "true",
    },
}

runtime_error_tests! {
    {
        name: invalid_argument,
        file: "cobra_own/invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: input_compare_3,
        file: "cobra_own/input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
}

static_error_tests! {
    {
        name: number_bounds_fail,
        file: "cobra_own/number_bounds_fail.snek",
        expected: "Invalid",
    }
}
