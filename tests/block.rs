#[macro_use]
mod common;

#[cfg(test)]
mod block {
    tests! {
        empty in block is OK
        "ok"
    }

    tests! {
        scope in block is OK
        "inner"
        "outer"
    }
}
