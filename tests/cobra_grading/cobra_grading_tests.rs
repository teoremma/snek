use crate::*;

// Your tests go here!
success_tests! {
    // Number and Boolean Literals
    {
        name: num,
        file: "cobra_grading/num.snek",
        expected: "644",
    },
    {
        name: false_val,
        file: "cobra_grading/false_val.snek",
        expected: "false",
    },

    // Input Expression
    {
        name: input_default,
        file: "cobra_grading/input0.snek",
        expected: "false",
    },
    {
        name: input_bool,
        file: "cobra_grading/input0.snek",
        input: "true",
        expected: "true",
    },
    {
        name: input_num,
        file: "cobra_grading/input0.snek",
        input: "123",
        expected: "123",
    },

    // Simple Number Expressions
    {
        name: add1,
        file: "cobra_grading/add1.snek",
        expected: "73",
    },
    {
        name: add1_sub1,
        file: "cobra_grading/add1_sub1.snek",
        expected: "4",
    },
    {
        name: add_num,
        file: "cobra_grading/add.snek",
        input: "10",
        expected: "15",
    },

    // Nested Arithmetic Expressions
    {
        name: nested_arith0,
        file: "cobra_grading/nested_arith0.snek",
        expected: "35",
    },
    {
        name: nested_arith1,
        file: "cobra_grading/nested_arith1.snek",
        expected: "25",
    },
    {
        name: nested_arith2,
        file: "cobra_grading/nested_arith2.snek",
        expected: "0",
    },
    {
        name: nested_arith3,
        file: "cobra_grading/nested_arith3.snek",
        input: "8",
        expected: "1117",
    },
    {
        name: nested_arith4,
        file: "cobra_grading/nested_arith4.snek",
        expected: "-1",
    },

    // Dynamic Type Checks with isnum/isbool
    {
        name: type_check_succ0,
        file: "cobra_grading/isnum.snek",
        expected: "false",
    },
    {
        name: type_check_succ1,
        file: "cobra_grading/isnum.snek",
        input: "547",
        expected: "true",
    },
    {
        name: type_check_succ2,
        file: "cobra_grading/isnum.snek",
        input: "true",
        expected: "false",
    },
    {
        name: type_check_succ3,
        file: "cobra_grading/isbool.snek",
        expected: "true",
    },
    {
        name: type_check_succ4,
        file: "cobra_grading/isbool.snek",
        input: "689",
        expected: "false",
    },
    {
        name: type_check_succ5,
        file: "cobra_grading/type_check_succ5.snek",
        expected: "true",
    },

    // Comparison Expressions
    {
        name: compare_expr_succ0,
        file: "cobra_grading/compare_expr_succ0.snek",
        expected: "true",
    },

    {
        name: compare_expr_succ2,
        file: "cobra_grading/compare_expr_succ2.snek",
        expected: "true",
    },

    // Let expressions
    {
        name: binding0,
        file: "cobra_grading/binding0.snek",
        expected: "5",
    },
    {
        name: binding1,
        file: "cobra_grading/binding1.snek",
        expected: "-5",
    },

    {
        name: binding_expr,
        file: "cobra_grading/binding_expr.snek",
        expected: "1225",
    },
    {
        name: binding_nested,
        file: "cobra_grading/binding_nested.snek",
        expected: "1",
    },

    {
        name: binding_chain,
        file: "cobra_grading/binding_chain.snek",
        expected: "3",
    },
    {
        name: binding_nested_chain,
        file: "cobra_grading/binding_nested_chain.snek",
        expected: "12",
    },

    // Let expressions with shadowing
    {
        name: shadowed_binding_succ0,
        file: "cobra_grading/shadowed_binding_succ0.snek",
        expected: "100",
    },
    {
        name: shadowed_binding_succ1,
        file: "cobra_grading/shadowed_binding_succ1.snek",
        expected: "7",
    },
    {
        name: shadowed_binding_succ2,
        file: "cobra_grading/shadowed_binding_succ2.snek",
        expected: "150",
    },
    {
        name: shadowed_binding_succ3,
        file: "cobra_grading/shadowed_binding_succ3.snek",
        expected: "5",
    },
    {
        name: shadowed_binding_succ4,
        file: "cobra_grading/shadowed_binding_succ4.snek",
        expected: "18",
    },
    {
        name: shadowed_binding_succ5,
        file: "cobra_grading/shadowed_binding_succ5.snek",
        expected: "5",
    },
    {
        name: shadowed_binding_succ6,
        file: "cobra_grading/shadowed_binding_succ6.snek",
        expected: "3",
    },
    {
        name: shadowed_binding_succ7,
        file: "cobra_grading/shadowed_binding_succ7.snek",
        expected: "200",
    },

    // Misc complex expressions with arithmetic and let bindings
    {
        name: complex_expr,
        file: "cobra_grading/complex_expr.snek",
        expected: "6",
    },
    {
        name: quick_brown_fox,
        file: "cobra_grading/quick_brown_fox.snek",
        expected: "-3776",
    },

    // If expressions
    {
        name: if_expr_succ0,
        file: "cobra_grading/if_expr_succ0.snek",
        expected: "10",
    },
    {
        name: if_expr_succ1,
        file: "cobra_grading/if_expr_input.snek",
        input: "635",
        expected: "20",
    },
    {
        name: if_expr_succ2,
        file: "cobra_grading/if_expr_succ2.snek",
        expected: "8",
    },
    {
        name: if_expr_succ3,
        file: "cobra_grading/if_expr_succ3.snek",
        expected: "7",
    },

    // Set expr
    {
        name: set_expr_succ0,
        file: "cobra_grading/set_expr1.snek",
        expected: "true",
    },
    {
        name: set_expr_succ1,
        file: "cobra_grading/set_expr2.snek",
        expected: "25",
    },
    {
        name: set_expr_succ2,
        file: "cobra_grading/set_expr3.snek",
        input: "25",
        expected: "true",
    },
    {
        name: set_expr_succ3,
        file: "cobra_grading/set_expr3.snek",
        input: "20",
        expected: "false",
    },

    {
        name: loop_expr_succ0,
        file: "cobra_grading/loop_expr0.snek",
        input: "3",
        expected: "6",
    },
    {
        name: loop_expr_succ1,
        file: "cobra_grading/loop_expr0.snek",
        input: "7",
        expected: "5040",
    },
    {
        name: loop_expr_succ2,
        file: "cobra_grading/loop_expr1.snek",
        expected: "-6",
    },
}

runtime_error_tests! {
    // integer overflow
    {
        name: number_overflow_fail0,
        file: "cobra_grading/number_overflow_fail0.snek",
        expected: "overflow",
    },
    {
        name: number_overflow_fail1,
        file: "cobra_grading/number_overflow_fail1.snek",
        expected: "overflow",
    },
    {
        name: number_overflow_fail2,
        file: "cobra_grading/add.snek",
        input: "4611686018427387899",
        expected: "overflow",
    },
    {
        name: number_overflow_fail3,
        file: "cobra_grading/nested_arith3.snek",
        input: "4611686018427387890",
        expected: "overflow",
    },

    // type mismatch
    {
        name: invalid_argument_fail0,
        file: "cobra_grading/invalid_argument_fail0.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail1,
        file: "cobra_grading/invalid_argument_fail1.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail2,
        file: "cobra_grading/invalid_argument_fail2.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail3,
        file: "cobra_grading/invalid_argument_fail3.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail4,
        file: "cobra_grading/invalid_argument_fail4.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail5,
        file: "cobra_grading/invalid_argument_fail5.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail6,
        file: "cobra_grading/invalid_argument_fail6.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail7,
        file: "cobra_grading/nested_arith3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail8,
        file: "cobra_grading/if_expr_input.snek",
        input: "665",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail9,
        file: "cobra_grading/set_expr3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail10,
        file: "cobra_grading/loop_expr0.snek",
        input: "5",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail11,
        file: "cobra_grading/invalid_argument_fail11.snek",
        expected: "invalid argument",
    },
}

static_error_tests! {

    // Invalid S-expressions
    {
        name: parse_sexp_fail1,
        file: "cobra_grading/parse_sexp_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_sexp_fail2,
        file: "cobra_grading/parse_sexp_fail2.snek",
        expected: "Invalid",
    },

    // Invalid tokens/operators
    {
        name: parse_token_fail1,
        file: "cobra_grading/parse_token_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail2,
        file: "cobra_grading/parse_token_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail3,
        file: "cobra_grading/parse_token_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail4,
        file: "cobra_grading/parse_token_fail4.snek",
        expected: "Invalid",
    },


    // Invalid/Out of bounds Number Literal
    {
        name: number_bounds_fail0,
        file: "cobra_grading/number_bounds_fail0.snek",
        expected: "Invalid",
    },
    {
        name: number_bounds_fail1,
        file: "cobra_grading/number_bounds_fail1.snek",
        expected: "Invalid",
    },

    // Invalid operator arguments
    {
        name: parse_op_fail1,
        file: "cobra_grading/parse_op_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail2,
        file: "cobra_grading/parse_op_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail3,
        file: "cobra_grading/parse_op_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fai4,
        file: "cobra_grading/parse_op_fail4.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail5,
        file: "cobra_grading/parse_op_fail5.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail6,
        file: "cobra_grading/parse_op_fail6.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail7,
        file: "cobra_grading/parse_op_fail7.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail8,
        file: "cobra_grading/parse_op_fail8.snek",
        expected: "Invalid",
    },

    // Invalid let expressions
    {
        name: parse_let_nobindings_fail,
        file: "cobra_grading/parse_let_nobindings_fail.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail1,
        file: "cobra_grading/parse_let_improperargs_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail2,
        file: "cobra_grading/parse_let_improperargs_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail3,
        file: "cobra_grading/parse_let_improperargs_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail4,
        file: "cobra_grading/parse_let_improperargs_fail4.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail5,
        file: "cobra_grading/parse_let_improperargs_fail5.snek",
        expected: "keyword",
    },

    {
        name: duplicate_binding_fail0,
        file: "cobra_grading/duplicate_binding_fail0.snek",
        expected: "Duplicate binding",
    },
    {
        name: duplicate_binding_fail1,
        file: "cobra_grading/duplicate_binding_fail1.snek",
        expected: "Duplicate binding",
    },
    {
        name: duplicate_binding_fail2,
        file: "cobra_grading/duplicate_binding_fail2.snek",
        expected: "Duplicate binding",
    },

    // Invalid if expressions
    {
        name: parse_if_fail0,
        file: "cobra_grading/parse_if_fail0.snek",
        expected: "Invalid",
    },
    {
        name: parse_if_fail1,
        file: "cobra_grading/parse_if_fail1.snek",
        expected: "Invalid",
    },

    // Unbound identifier
    {
        name: unbound_identifier_fail0,
        file: "cobra_grading/unbound_identifier_fail0.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: unbound_identifier_fail1,
        file: "cobra_grading/unbound_identifier_fail1.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: unbound_identifier_fail2,
        file: "cobra_grading/unbound_identifier_fail2.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: unbound_identifier_fail3,
        file: "cobra_grading/unbound_identifier_fail3.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: unbound_identifier_fail4,
        file: "cobra_grading/unbound_identifier_fail4.snek",
        expected: "Unbound variable identifier t",
    },
    {
        name: unbound_identifier_fail5,
        file: "cobra_grading/unbound_identifier_fail5.snek",
        expected: "Unbound variable identifier x",
    },

    // Invalid block
    {
        name: parse_block_fail0,
        file: "cobra_grading/parse_block_fail0.snek",
        expected: "Invalid",
    },

    // Invalid break
    {
        name: invalid_break_fail0,
        file: "cobra_grading/invalid_break_fail0.snek",
        expected: "break",
    },

    // Invalid loop
    {
        name: invalid_loop_fail0,
        file: "cobra_grading/invalid_loop_fail0.snek",
        expected: "Invalid",
    }
}
