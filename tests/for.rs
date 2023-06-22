#[macro_use]
mod common;

#[cfg(test)]
mod _for {
    tests! {
        class_in_body in for is ERR
        "[line 2:10] Error at 'class': Expected expression"
    }

    tests! {
        closure_in_body in for is OK
        "4"
        "1"
        "4"
        "2"
        "4"
        "3"
    }

    tests! {
        fun_in_body in for is ERR
        "[line 2:10] Error at 'fun': Expected expression"
    }

    tests! {
        return_closure in for is OK
        "i"
    }

    tests! {
        return_inside in for is OK
        "i"
    }
}
