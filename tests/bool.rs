#[macro_use]
mod common;

#[cfg(test)]
mod bool {
    tests! {
        equality in bool is OK
        "true"
        "false"
        "false"
        "true"
        "false"
        "true"
        "true"
        "false"
    }

    tests! {
        mismatched in bool is ERR
        "[line 1:6] Error at '==': Binary operation with mismatched literal types is not supported"
        "[line 2:7] Error at '==': Binary operation with mismatched literal types is not supported"
        "[line 3:6] Error at '!=': Binary operation with mismatched literal types is not supported"
    }

    tests! {
        not in bool is OK
        "false"
        "true"
        "true"
    }
}
