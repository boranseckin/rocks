#[macro_use]
mod common;

#[cfg(test)]
mod constructor {
    tests! {
        arguments in constructor is OK
        "init"
        "1"
        "2"
    }

    tests! {
        call_init_early_return in constructor is OK
        "init"
        "init"
        "<instance Foo>"
    }

    tests! {
        call_init_explicitly in constructor is OK
        "Foo.init(one)"
        "Foo.init(two)"
        "<instance Foo>"
        "init"
    }

    tests! {
        default in constructor is OK
        "<instance Foo>"
    }

    tests! {
        default_arguments in constructor is ERR
        "[line 3:22] Error at ')': Expected 0 arguments but got 3"
    }

    tests! {
        early_return in constructor is OK
        "init"
        "<instance Foo>"
    }

    tests! {
        extra_arguments in constructor is ERR
        "[line 8:25] Error at ')': Expected 2 arguments but got 4"
    }

    tests! {
        init_not_method in constructor is OK
        "not initializer"
    }

    tests! {
        missing_arguments in constructor is ERR
        "[line 5:16] Error at ')': Expected 2 arguments but got 1"
    }

    tests! {
        return_in_nested_function in constructor is OK
        "bar"
        "<instance Foo>"
    }

    tests! {
        return_value in constructor is ERR
        "[line 3:5] Error at 'return': Cannot return a value from an initializer"
    }
}
