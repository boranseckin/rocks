#[macro_use]
mod common;

#[cfg(test)]
mod comment {
    tests! {
        line_at_eof in comment is OK
        "ok"
    }

    tests! {
        only_line_comment in comment is OK
    }

    tests! {
        only_line_comment_and_line in comment is OK
    }

    tests! {
        unicode in comment is OK
        "ok"
    }
}
