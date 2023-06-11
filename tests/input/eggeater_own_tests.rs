use crate::*;

// Your tests go here!
success_tests! {
    {
        name: simple_examples_1,
        file: "input/simple_examples.snek",
        input: "0",
        expected: "1",
    },
    {
        name: simple_examples_2,
        file: "input/simple_examples.snek",
        input: "1",
        expected: "10",
    },
    {
        name: simple_examples_3,
        file: "input/simple_examples.snek",
        input: "2",
        expected: "100",
    },
    {
        name: points1,
        file: "input/points.snek",
        input: "1",
        expected: "11\n22\n0",
    },
    {
        name: points3,
        file: "input/points.snek",
        input: "3",
        expected: "33\n66\n0",
    },
}

runtime_error_tests! {
    {
        name: index_invalid_tuple,
        file: "input/index_invalid_tuple.snek",
        expected: "tuple",
    },
    {
        name: index_invalid_index,
        file: "input/index_invalid_index.snek",
        expected: "invalid",
    },
}

static_error_tests! {}
