#[macro_use]
mod common;

#[cfg(test)]
mod bool {
    tests! {
        equality in bool is OK
        "true"
        "false"
        "false"
        "true"
        "false"
        "true"
        "true"
        "false"
    }

    tests! {
        mismatched in bool is OK
        "false"
        "false"
        "true"
    }

    tests! {
        not in bool is OK
        "false"
        "true"
        "true"
    }
}
