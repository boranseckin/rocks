#[macro_use]
mod common;

#[cfg(test)]
mod null {
    tests! {
        literal in null is OK
        "null"
    }
}
