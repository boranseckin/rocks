#[macro_use]
mod common;

#[cfg(test)]
mod variable {
    tests! {
        collide_with_parameter in variable is ERR
        "[line 2:7] Error at 'a': A variable is already defined with name 'a' in this scope"
    }

    tests! {
        duplicate_local in variable is ERR
        "[line 3:7] Error at 'a': A variable is already defined with name 'a' in this scope"
    }

    tests! {
        duplicate_parameter in variable is ERR
        "[line 1:14] Error at 'arg': A variable is already defined with name 'arg' in this scope"
    }

    tests! {
        early_bound in variable is OK
        "outer"
        "outer"
    }

    tests! {
        in_middle_of_block in variable is OK
        "a"
        "a b"
        "a c"
        "a b d"
    }

    tests! {
        in_nested_block in variable is OK
        "outer"
    }

    tests! {
        local_from_method in variable is OK
        "variable"
    }

    tests! {
        redeclare_global in variable is OK
        "null"
    }

    tests! {
        redefine_global in variable is OK
        "2"
    }

    tests! {
        scope_reuse_in_different_blocks in variable is OK
        "first"
        "second"
    }

    tests! {
        shadow_and_local in variable is OK
        "outer"
        "inner"
    }

    tests! {
        shadow_global in variable is OK
        "shadow"
        "global"
    }

    tests! {
        shadow_local in variable is OK
        "shadow"
        "local"
    }

    tests! {
        undefined_global in variable is ERR
        "[line 1:7] Error at 'notDefined': Undefined variable 'notDefined'"
    }

    tests! {
        undefined_local in variable is ERR
        "[line 2:9] Error at 'notDefined': Undefined variable 'notDefined'"
    }

    tests! {
        uninitialized in variable is OK
        "null"
    }

    tests! {
        unreached_undefined in variable is OK
        "ok"
    }

    tests! {
        use_false_as_var in variable is ERR
        "[line 2:5] Error at 'false': Expected variable name"
    }

    tests! {
        use_global_in_initializer in variable is OK
        "value"
    }

    tests! {
        use_local_in_initializer in variable is ERR
        "[line 3:11] Error at 'a': Cannot read local variable in its own initializer"
    }

    tests! {
        use_null_as_var in variable is ERR
        "[line 2:5] Error at 'null': Expected variable name"
    }

    tests! {
        use_this_as_var in variable is ERR
        "[line 2:5] Error at 'this': Expected variable name"
    }
}
