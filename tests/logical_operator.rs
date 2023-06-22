#[macro_use]
mod common;

#[cfg(test)]
mod logical_operator {
    tests! {
        and in logical_operator is OK
        "false"
        "1"
        "false"
        "true"
        "3"
        "true"
        "false"
    }

    tests! {
        and_truth in logical_operator is OK
        "false"
        "null"
        ""
        "0"
        "ok"
        "ok"
        "ok"
    }

    tests! {
        or in logical_operator is OK
        "1"
        "1"
        "true"
        "false"
        "false"
        "false"
        "true"
    }

    tests! {
        or_truth in logical_operator is OK
        "ok"
        "ok"
        "ok"
        "ok"
        "true"
        "1"
        "s"
    }
}
