#[macro_use]
mod common;

#[cfg(test)]
mod _super {
    tests! {
        bound_method in super is OK
        "A.method(arg)"
    }

    tests! {
        call_other_method in super is OK
        "Derived.bar()"
        "Base.foo()"
    }

    tests! {
        call_same_method in super is OK
        "Derived.foo()"
        "Base.foo()"
    }

    tests! {
        closure in super is OK
        "Base"
    }

    tests! {
        constructor in super is OK
        "Derived.init()"
        "Base.init(a, b)"
    }

    tests! {
        extra_arguments in super is ERR
        "[line 10:33] Error at ')': Expected 2 arguments but got 4"
    }

    tests! {
        indirectly_inherited in super is OK
        "C.foo()"
        "A.foo()"
    }

    tests! {
        missing_arguments in super is ERR
        "[line 9:16] Error at ')': Expected 2 arguments but got 1"
    }

    tests! {
        no_superclass_bind in super is ERR
        "[line 3:5] Error at 'super': Cannot use 'super' in a class with no superclass"
    }

    tests! {
        no_superclass_call in super is ERR
        "[line 3:5] Error at 'super': Cannot use 'super' in a class with no superclass"
    }

    tests! {
        no_superclass_method in super is ERR
        "[line 5:11] Error at 'doesNotExist': Undefined property 'doesNotExist'"
    }

    tests! {
        parenthesized in super is ERR
        "[line 8:11] Error at ')': Expected '.' after 'super'"
    }

    tests! {
        reassign_superclass in super is OK
        "Base.method()"
        "Base.method()"
    }

    tests! {
        super_at_top_level in super is ERR
        "[line 1:1] Error at 'super': Cannot use 'super' outside of a class"
        "[line 2:1] Error at 'super': Cannot use 'super' outside of a class"
    }

    tests! {
        super_in_closure_in_inherited_method in super is OK
        "A"
    }

    tests! {
        super_in_inherited_method in super is OK
        "A"
    }

    tests! {
        super_in_top_level_function in super is ERR
        "[line 1:1] Error at 'super': Cannot use 'super' outside of a class"
    }

    tests! {
        super_without_dot in super is ERR
        "[line 6:10] Error at ';': Expected '.' after 'super'"
    }

    tests! {
        super_without_name in super is ERR
        "[line 5:11] Error at ';': Expected superclass method name"
    }

    tests! {
        this_in_superclass_method in super is OK
        "a"
        "b"
    }
}
