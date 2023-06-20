#[macro_use]
mod common;

#[cfg(test)]
mod _return {
    tests! {
        after_else in return is OK
        "ok"
    }

    tests! {
        after_if in return is OK
        "ok"
    }

    tests! {
        after_while in return is OK
        "ok"
    }

    tests! {
        at_top_level in return is ERR
        "[line 1:1] Error at 'return': Cannot return from top-level code"
    }

    tests! {
        in_function in return is OK
        "ok"
    }

    tests! {
        in_method in return is OK
        "ok"
    }

    tests! {
        return_null_if_no_value in return is OK
        "null"
    }
}
