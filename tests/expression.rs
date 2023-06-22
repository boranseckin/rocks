#[macro_use]
mod common;

#[cfg(test)]
mod expression {
    tests! {
        evaluate in expression is OK
        "2"
    }
}
