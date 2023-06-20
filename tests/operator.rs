#[macro_use]
mod common;

#[cfg(test)]
mod operator {
    tests! {
        add in operator is OK
        "579"
        "string"
    }

    tests! {
        add_bool_null in operator is ERR
        "[line 1:6] Error at '+': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        add_bool_num in operator is ERR
        "[line 1:6] Error at '+': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        add_bool_string in operator is ERR
        "[line 1:6] Error at '+': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        add_null_null in operator is ERR
        "[line 1:6] Error at '+': Binary operation '+' is not supported for null type"
    }

    tests! {
        add_num_null in operator is ERR
        "[line 1:3] Error at '+': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        add_string_null in operator is ERR
        "[line 1:5] Error at '+': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        comparison in operator is OK
        "true"
        "false"
        "false"
        "true"
        "true"
        "false"
        "false"
        "false"
        "true"
        "false"
        "true"
        "true"
        "false"
        "false"
        "false"
        "false"
        "true"
        "true"
        "true"
        "true"
    }

    tests! {
        divide in operator is OK
        "4"
        "1"
    }

    tests! {
        divide_nonnum_num in operator is ERR
        "[line 1:5] Error at '/': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        divide_num_nonnum in operator is ERR
        "[line 1:3] Error at '/': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        equals in operator is OK
        "true"
        "true"
        "false"
        "true"
        "false"
        "true"
        "false"
    }

    // non-literal comparisons are not supported
    // tests! {
    //     equals_class in operator is OK
    //     "true"
    //     "false"
    //     "false"
    //     "true"
    // }

    // tests! {
    //     equals_method in operator is OK
    //     "true"
    //     "false"
    // }

    tests! {
        greater_nonnum_num in operator is ERR
        "[line 1:5] Error at '>': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        greater_num_nonnum in operator is ERR
        "[line 1:3] Error at '>': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        greater_or_equal_nonnum_num in operator is ERR
        "[line 1:5] Error at '>=': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        greater_or_equal_num_nonnum in operator is ERR
        "[line 1:3] Error at '>=': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        less_nonnum_num in operator is ERR
        "[line 1:5] Error at '<': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        less_num_nonnum in operator is ERR
        "[line 1:3] Error at '<': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        less_or_equal_nonnum_num in operator is ERR
        "[line 1:5] Error at '<=': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        less_or_equal_num_nonnum in operator is ERR
        "[line 1:3] Error at '<=': Binary operation with mismatched literal types is not supported"
    }

    // floating point arithmetics are hard :(
    tests! {
        multiply in operator is OK
        "15"
        "3.7020001"
    }

    tests! {
        multiply_nonnum_num in operator is ERR
        "[line 1:5] Error at '*': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        multiply_num_nonnum in operator is ERR
        "[line 1:3] Error at '*': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        negate in operator is OK
        "-3"
        "3"
        "-3"
    }

    tests! {
        negate_nonnum in operator is ERR
        "[line 1:7] Error at '-': Unary operation '-' is not supported for non-number types"
    }

    tests! {
        not in operator is OK
        "false"
        "true"
        "true"
        "false"
        "true"
        "true"
        "true"
    }

    // non-literal operations are not supported
    // tests! {
    //     not_class in operator is OK
    //     "false"
    //     "false"
    // }

    tests! {
        not_equals in operator is OK
        "false"
        "false"
        "true"
        "false"
        "true"
        "false"
        "true"
    }

    tests! {
        subtract in operator is OK
        "1"
        "0"
    }

    tests! {
        subtract_nonnum_num in operator is ERR
        "[line 1:5] Error at '-': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        subtract_num_nonnum in operator is ERR
        "[line 1:3] Error at '-': Binary operation with mismatched literal types is not supported"
    }
}
