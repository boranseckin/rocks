#[macro_use]
mod common;

#[cfg(test)]
mod closure {
    tests! {
        assign_to_closure in closure is OK
        "local"
        "after f"
        "after f"
        "after g"
    }

    tests! {
        assign_to_shadowed_later in closure is OK
        "inner"
        "assigned"
    }

    tests! {
        close_over_function_parameter in closure is OK
        "param"
    }

    tests! {
        close_over_later_variable in closure is OK
        "b"
        "a"
    }

    tests! {
        close_over_method_parameter in closure is OK
        "param"
    }

    tests! {
        closed_closure_in_function in closure is OK
        "local"
    }

    tests! {
        nested_closure in closure is OK
        "a"
        "b"
        "c"
    }

    tests! {
        open_closure_in_function in closure is OK
        "local"
    }

    tests! {
        reference_closure_multiple_times in closure is OK
        "a"
        "a"
    }

    tests! {
        reuse_closure_slot in closure is OK
        "a"
    }

    tests! {
        shadow_closure_with_local in closure is OK
        "closure"
        "shadow"
        "closure"
    }

    tests! {
        unused_closure in closure is OK
        "ok"
    }

    tests! {
        unused_later_closure in closure is OK
        "a"
    }
}
