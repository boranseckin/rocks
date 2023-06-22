#[macro_use]
mod common;

#[cfg(test)]
mod number {
    tests! {
        decimal_point_at_eof in number is ERR
        "[line 2:1] Error: Unterminated number"
    }

    tests! {
        leading_dot in number is ERR
        "[line 2:1] Error at '.': Expected expression"
    }

    tests! {
        literals in number is OK
        "123"
        "987654"
        "0"
        "-0"
        "123.456"
        "-0.001"
    }

    tests! {
        nan_equality in number is OK
        "false"
        "true"
        "false"
        "true"
    }

    tests! {
        trailing_dot in number is ERR
        "[line 2:1] Error: Unterminated number"
    }
}
