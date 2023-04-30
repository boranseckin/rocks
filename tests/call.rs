#[macro_use]
mod common;

#[cfg(test)]
mod call {
    tests! {
        bool in call is ERR
        "[line 0:5] Error at ')': Can only call functions and classes"
    }

    tests! {
        null in call is ERR
        "[line 0:5] Error at ')': Can only call functions and classes"
    }

    tests! {
        num in call is ERR
        "[line 0:4] Error at ')': Can only call functions and classes"
    }

    tests! {
        object in call is ERR
        "[line 3:4] Error at ')': Can only call functions and classes"
    }

    tests! {
        string in call is ERR
        "[line 0:6] Error at ')': Can only call functions and classes"
    }
}
