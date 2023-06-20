#[macro_use]
mod common;

#[cfg(test)]
mod function {
    tests! {
        body_must_be_block in function is ERR
        "[line 3:9] Error at '123': Expected '{' before function body"
    }

    tests! {
        empty_body in function is OK
        "null"
    }

    tests! {
        extra_arguments in function is ERR
        "[line 6:13] Error at ')': Expected 2 arguments but got 4"
    }

    tests! {
        local_mutual_recursion in function is ERR
        "[line 4:12] Error at 'isOdd': Undefined variable 'isOdd'"
        "[line 4:23] Error at ')': Can only call functions and classes"
    }

    tests! {
        local_recursion in function is OK
        "21"
    }

    tests! {
        missing_arguments in function is ERR
        "[line 3:4] Error at ')': Expected 2 arguments but got 1"
    }

    tests! {
        missing_comma_in_parameters in function is ERR
        "[line 2:14] Error at 'c': Expected ')' after parameters"
    }

    tests! {
        mutual_recursion in function is OK
        "true"
        "true"
    }

    tests! {
        nested_call_with_arguments in function is OK
        "hello world"
    }

    tests! {
        parameters in function is OK
        "0"
        "1"
        "3"
        "6"
        "10"
        "15"
        "21"
        "28"
        "36"
    }

    tests! {
        print in function is OK
        "<function foo>"
        "<native function clock>"
    }

    tests! {
        recursion in function is OK
        "21"
    }

    tests! {
        too_many_arguments in function is ERR
        "[line 260:6] Error at 'a': Cannot have more than 255 arguments"
    }

    tests! {
        too_many_parameters in function is ERR
        "[line 257:11] Error at 'a': Cannot have more than 255 parameters"
    }
}

