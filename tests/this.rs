#[macro_use]
mod common;

#[cfg(test)]
mod this {
    tests! {
        closure in this is OK
        "Foo"
    }

    tests! {
        nested_class in this is OK
        "<instance Outer>"
        "<instance Outer>"
        "<instance Inner>"
    }

    tests! {
        nested_closure in this is OK
        "Foo"
    }

    tests! {
        this_at_top_level in this is ERR
        "[line 1:1] Error at 'this': Cannot use 'this' outside of a class"
    }

    tests! {
        this_in_method in this is OK
        "baz"
    }

    tests! {
        this_in_top_level_function in this is ERR
        "[line 2:3] Error at 'this': Cannot use 'this' outside of a class"
    }
}
