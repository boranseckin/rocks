#[macro_use]
mod common;

#[cfg(test)]
mod string {
    tests! {
        error_after_multiline in string is ERR
        "[line 7:1] Error at 'err': Undefined variable 'err'"
    }

    tests! {
        literals in string is OK
        "()"
        "a string"
        "A~¶Þॐஃ"
    }

    tests! {
        multiline in string is OK
        "1"
        "2"
        "3"
    }

    tests! {
        unterminated in string is ERR
        "[line 2:1] Error: Unterminated string"
    }
}
