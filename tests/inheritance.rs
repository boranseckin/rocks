#[macro_use]
mod common;

#[cfg(test)]
mod inheritance {
    tests! {
        constructor in inheritance is OK
        "value"
    }

    tests! {
        inherit_from_function in inheritance is ERR
        "[line 3:7] Error at 'Subclass': Superclass must be a class"
    }

    tests! {
        inherit_from_null in inheritance is ERR
        "[line 2:7] Error at 'Foo': Superclass must be a class"
    }

    tests! {
        inherit_from_number in inheritance is ERR
        "[line 2:7] Error at 'Foo': Superclass must be a class"
    }

    tests! {
        inherit_methods in inheritance is OK
        "foo"
        "bar"
        "bar"
    }

    tests! {
        parenthesized_superclass in inheritance is ERR
        "[line 4:13] Error at '(': Expected superclass name"
    }

    tests! {
        set_fields_from_base_class in inheritance is OK
        "foo 1"
        "foo 2"
        "bar 1"
        "bar 2"
        "bar 1"
        "bar 2"
    }
}
