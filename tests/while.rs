#[macro_use]
mod common;

#[cfg(test)]
mod _while {
    tests! {
        class_in_body in while is ERR
        "[line 2:14] Error at 'class': Expected expression"
    }

    tests! {
        closure_in_body in while is OK
        "1"
        "2"
        "3"
    }

    tests! {
        fun_in_body in while is ERR
        "[line 2:14] Error at 'fun': Expected expression"
    }

    tests! {
        return_closure in while is OK
        "i"
    }

    tests! {
        return_inside in while is OK
        "i"
    }

    tests! {
        syntax in while is OK
        "1"
        "2"
        "3"
        "0"
        "1"
        "2"
    }

    tests! {
        var_in_body in while is ERR
        "[line 2:14] Error at 'var': Expected expression"
    }
}
