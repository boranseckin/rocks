#[macro_use]
mod common;

#[cfg(test)]
mod print {
    tests! {
        missing_argument in print is ERR
        "[line 2:6] Error at ';': Expected expression"
    }
}
