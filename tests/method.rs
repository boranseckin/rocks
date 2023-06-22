#[macro_use]
mod common;

#[cfg(test)]
mod method {
    tests! {
        arity in method is OK
        "no args"
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
        empty_block in method is OK
        "null"
    }

    tests! {
        extra_arguments in method is ERR
        "[line 8:24] Error at ')': Expected 2 arguments but got 4"
    }

    tests! {
        missing_arguments in method is ERR
        "[line 5:15] Error at ')': Expected 2 arguments but got 1"
    }

    tests! {
        not_found in method is ERR
        "[line 3:7] Error at 'unknown': Undefined property 'unknown'"
    }

    tests! {
        print_bound_method in method is OK
        "<function method>"
    }

    tests! {
        refer_to_name in method is ERR
        "[line 3:11] Error at 'method': Undefined variable 'method'"
    }

    tests! {
        too_many_arguments in method is ERR
        "[line 259:6] Error at 'a': Cannot have more than 255 arguments"
    }

    tests! {
        too_many_parameters in method is ERR
        "[line 258:11] Error at 'a': Cannot have more than 255 parameters"
    }
}
