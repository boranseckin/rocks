#[macro_use]
mod common;

#[cfg(test)]
mod assignment {
    tests! {
        associativity in assignment is OK
        "c"
        "c"
        "c"
    }

    tests! {
        global in assignment is OK
        "before"
        "after"
        "arg"
        "arg"
    }

    tests! {
        grouping in assignment is ERR
        "[line 2:5] Error at '=': Invalid assignment target"
    }

    tests! {
        infix_operator in assignment is ERR
        "[line 3:7] Error at '=': Invalid assignment target"
    }

    tests! {
        local in assignment is OK
        "before"
        "after"
        "arg"
        "arg"
    }

    tests! {
        prefix_operator in assignment is ERR
        "[line 2:4] Error at '=': Invalid assignment target"
    }

    tests! {
        syntax in assignment is OK
        "var"
        "var"
    }

    tests! {
        to_this in assignment is ERR
        "[line 3:10] Error at '=': Invalid assignment target"
    }

    tests! {
        undefined in assignment is ERR
        "[line 1:1] Error at 'unknown': Undefined variable 'unknown'"
    }
}
