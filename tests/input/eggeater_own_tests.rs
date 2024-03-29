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
    {
        name: set_last,
        file: "input/set_last.snek",
        expected: "(1, 2, 4)",
    },
    {
        name: iterate,
        file: "input/iterate.snek",
        expected: "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n100",
    },
    {
        name: print_tuple_simple,
        file: "input/print_tuple_simple.snek",
        expected: "(1, 2, 3)",
    },
    {
        name: print_tuple_cycle,
        file: "input/print_tuple_cycle.snek",
        expected: "(1, (2, (...)))",
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
