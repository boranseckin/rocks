#[macro_use]
mod common;

#[cfg(test)]
mod misc {
    tests! {
        empty_file in misc is OK
    }

    tests! {
        precedence in misc is OK
        "14"
        "8"
        "4"
        "0"
        "true"
        "true"
        "true"
        "true"
        "0"
        "0"
        "0"
        "0"
        "4"
    }

    tests! {
        unexpected_character in misc is ERR
        "[line 3:7] Error: Unexpected character '|'"
    }
}
