#[macro_use]
mod common;

#[cfg(test)]
mod call {
    tests! {
        bool in call is ERR
        "[line 1:6] Error at ')': Can only call functions and classes"
    }

    tests! {
        null in call is ERR
        "[line 1:6] Error at ')': Can only call functions and classes"
    }

    tests! {
        num in call is ERR
        "[line 1:5] Error at ')': Can only call functions and classes"
    }

    tests! {
        object in call is ERR
        "[line 4:5] Error at ')': Can only call functions and classes"
    }

    tests! {
        string in call is ERR
        "[line 1:7] Error at ')': Can only call functions and classes"
    }
}
