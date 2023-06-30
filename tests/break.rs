#[macro_use]
mod common;

#[cfg(test)]
mod _break {
    tests! {
        inside_while in break is OK
        "0"
        "1"
        "2"
    }

    tests! {
        inside_for in break is OK
        "0"
        "1"
        "2"
    }

    tests! {
        nested in break is OK
        "inside"
        "outside"
    }

    tests! {
        no_loop in break is ERR
        "[line 2:3] Error at 'break': Cannot break outside of a loop"
    }
}
