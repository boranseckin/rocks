#[macro_use]
mod common;

#[cfg(test)]
mod class {
    tests! {
        empty in class is OK
        "<class Foo>"
    }

    tests! {
        inherit_self in class is ERR
        "[line 0:12] Error at 'Foo': A class cannot inherit from itself"
    }

    tests! {
        inherited_method in class is OK
        "in foo"
        "in bar"
        "in baz"
    }

    tests! {
        local_inherit_other in class is OK
        "<class B>"
    }

    tests! {
        local_inherit_self in class is ERR
        "[line 1:14] Error at 'Foo': A class cannot inherit from itself"
    }

    tests! {
        local_reference_self in class is OK
        "<class Foo>"
    }

    tests! {
        reference_self in class is OK
        "<class Foo>"
    }
}
