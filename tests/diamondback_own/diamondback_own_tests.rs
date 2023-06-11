use crate::*;

// Your tests go here!
success_tests! {
    {
        name: fact,
        file: "diamondback_own/fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: even_odd_1,
        file: "diamondback_own/even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "diamondback_own/even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: sum4,
        file: "diamondback_own/sum4.snek",
        expected: "1234",
    }
}

runtime_error_tests! {}

static_error_tests! {
    {
        name: duplicate_params,
        file: "diamondback_own/duplicate_params.snek",
        expected: "",
    }
}
