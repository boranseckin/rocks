#[macro_use]
mod common;

#[cfg(test)]
mod _if {
    tests! {
        class_in_else in if is ERR
        "[line 2:22] Error at 'class': Expected expression"
    }

    tests! {
        class_in_then in if is ERR
        "[line 2:11] Error at 'class': Expected expression"
    }

    tests! {
        dangling_else in if is OK
        "good"
    }

    tests! {
        else_flow in if is OK
        "good"
        "good"
        "block"
    }

    tests! {
        fun_in_else in if is ERR
        "[line 2:22] Error at 'fun': Expected expression"
    }

    tests! {
        fun_in_then in if is ERR
        "[line 2:11] Error at 'fun': Expected expression"
    }

    tests! {
        if_flow in if is OK
        "good"
        "block"
        "true"
    }

    tests! {
        truth in if is OK
        "false"
        "null"
        "true"
        "0"
        "empty"
    }

    tests! {
        var_in_else in if is ERR
        "[line 2:22] Error at 'var': Expected expression"
    }

    tests! {
        var_in_then in if is ERR
        "[line 2:11] Error at 'var': Expected expression"
    }
}
