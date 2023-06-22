#[macro_use]
mod common;

#[cfg(test)]
mod regression {
    tests! {
        b394 in regression is OK
        "<class B>"
    }

    tests! {
        b40 in regression is OK
        "<function f>"
    }
}
