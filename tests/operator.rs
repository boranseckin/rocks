#[macro_use]
mod common;

#[cfg(test)]
mod operator {
    tests! {
        add in operator is OK
        "0"
        "579"
        "string"
        "str"
        ""
    }

    tests! {
        add_literal in operator is ERR
        "[line 1:6] Error at '+': Binary operation '+' is not supported between boolean type and null type"
        "[line 2:6] Error at '+': Binary operation '+' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '+': Binary operation '+' is not supported between boolean type and string type"
        "[line 4:6] Error at '+': Binary operation '+' is not supported between boolean type and number type"
        "[line 6:6] Error at '+': Binary operation '+' is not supported between null type and null type"
        "[line 7:6] Error at '+': Binary operation '+' is not supported between null type and boolean type"
        "[line 8:6] Error at '+': Binary operation '+' is not supported between null type and string type"
        "[line 9:6] Error at '+': Binary operation '+' is not supported between null type and number type"
        "[line 11:5] Error at '+': Binary operation '+' is not supported between number type and null type"
        "[line 12:5] Error at '+': Binary operation '+' is not supported between number type and boolean type"
        "[line 13:5] Error at '+': Binary operation '+' is not supported between number type and string type"
        "[line 15:5] Error at '+': Binary operation '+' is not supported between string type and null type"
        "[line 16:5] Error at '+': Binary operation '+' is not supported between string type and boolean type"
        "[line 17:5] Error at '+': Binary operation '+' is not supported between string type and number type"
    }

    tests! {
        add_non_literal in operator is ERR
        "[line 5:11] Error at '+': Binary operation '+' is not supported between number type and function type"
        "[line 6:11] Error at '+': Binary operation '+' is not supported between number type and class type"
        "[line 7:11] Error at '+': Binary operation '+' is not supported between number type and instance type"
        "[line 8:11] Error at '+': Binary operation '+' is not supported between number type and native function type"
    }

    tests! {
        comparison in operator is OK
        "true"
        "false"
        "false"
        "true"
        "true"
        "false"
        "false"
        "false"
        "true"
        "false"
        "true"
        "true"
        "false"
        "false"
        "false"
        "false"
        "true"
        "true"
        "true"
        "true"
    }

    tests! {
        divide in operator is OK
        "4"
        "1"
        "inf"
    }

    tests! {
        divide_literal in operator is ERR
        "[line 1:6] Error at '/': Binary operation '/' is not supported between boolean type and null type"
        "[line 2:6] Error at '/': Binary operation '/' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '/': Binary operation '/' is not supported between boolean type and string type"
        "[line 4:6] Error at '/': Binary operation '/' is not supported between boolean type and number type"
        "[line 6:6] Error at '/': Binary operation '/' is not supported between null type and null type"
        "[line 7:6] Error at '/': Binary operation '/' is not supported between null type and boolean type"
        "[line 8:6] Error at '/': Binary operation '/' is not supported between null type and string type"
        "[line 9:6] Error at '/': Binary operation '/' is not supported between null type and number type"
        "[line 11:5] Error at '/': Binary operation '/' is not supported between number type and null type"
        "[line 12:5] Error at '/': Binary operation '/' is not supported between number type and boolean type"
        "[line 13:5] Error at '/': Binary operation '/' is not supported between number type and string type"
        "[line 15:5] Error at '/': Binary operation '/' is not supported between string type and null type"
        "[line 16:5] Error at '/': Binary operation '/' is not supported between string type and boolean type"
        "[line 17:5] Error at '/': Binary operation '/' is not supported between string type and number type"
    }

    tests! {
        divide_non_literal in operator is ERR
        "[line 5:11] Error at '/': Binary operation '/' is not supported between number type and function type"
        "[line 6:11] Error at '/': Binary operation '/' is not supported between number type and class type"
        "[line 7:11] Error at '/': Binary operation '/' is not supported between number type and instance type"
        "[line 8:11] Error at '/': Binary operation '/' is not supported between number type and native function type"
    }

    tests! {
        equals in operator is OK
        "true"
        "true"
        "false"
        "true"
        "false"
        "true"
        "false"
    }

    tests! {
        equals_uninitialized in operator is OK
        "true"
        "true"
    }

    tests! {
        equals_class in operator is OK
        "true"
        "false"
        "false"
        "true"
    }

    tests! {
        equals_method in operator is OK
        "true"
    }

    tests! {
        greater_literal in operator is ERR
        "[line 1:6] Error at '>': Binary operation '>' is not supported between boolean type and null type"
        "[line 2:6] Error at '>': Binary operation '>' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '>': Binary operation '>' is not supported between boolean type and string type"
        "[line 4:6] Error at '>': Binary operation '>' is not supported between boolean type and number type"
        "[line 6:6] Error at '>': Binary operation '>' is not supported between null type and null type"
        "[line 7:6] Error at '>': Binary operation '>' is not supported between null type and boolean type"
        "[line 8:6] Error at '>': Binary operation '>' is not supported between null type and string type"
        "[line 9:6] Error at '>': Binary operation '>' is not supported between null type and number type"
        "[line 11:5] Error at '>': Binary operation '>' is not supported between number type and null type"
        "[line 12:5] Error at '>': Binary operation '>' is not supported between number type and boolean type"
        "[line 13:5] Error at '>': Binary operation '>' is not supported between number type and string type"
        "[line 15:5] Error at '>': Binary operation '>' is not supported between string type and null type"
        "[line 16:5] Error at '>': Binary operation '>' is not supported between string type and boolean type"
        "[line 17:5] Error at '>': Binary operation '>' is not supported between string type and number type"
    }

    tests! {
        greater_non_literal in operator is ERR
        "[line 5:11] Error at '>': Binary operation '>' is not supported between number type and function type"
        "[line 6:11] Error at '>': Binary operation '>' is not supported between number type and class type"
        "[line 7:11] Error at '>': Binary operation '>' is not supported between number type and instance type"
        "[line 8:11] Error at '>': Binary operation '>' is not supported between number type and native function type"
    }

    tests! {
        greater_or_equal_literal in operator is ERR
        "[line 1:6] Error at '>=': Binary operation '>=' is not supported between boolean type and null type"
        "[line 2:6] Error at '>=': Binary operation '>=' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '>=': Binary operation '>=' is not supported between boolean type and string type"
        "[line 4:6] Error at '>=': Binary operation '>=' is not supported between boolean type and number type"
        "[line 6:6] Error at '>=': Binary operation '>=' is not supported between null type and null type"
        "[line 7:6] Error at '>=': Binary operation '>=' is not supported between null type and boolean type"
        "[line 8:6] Error at '>=': Binary operation '>=' is not supported between null type and string type"
        "[line 9:6] Error at '>=': Binary operation '>=' is not supported between null type and number type"
        "[line 11:5] Error at '>=': Binary operation '>=' is not supported between number type and null type"
        "[line 12:5] Error at '>=': Binary operation '>=' is not supported between number type and boolean type"
        "[line 13:5] Error at '>=': Binary operation '>=' is not supported between number type and string type"
        "[line 15:5] Error at '>=': Binary operation '>=' is not supported between string type and null type"
        "[line 16:5] Error at '>=': Binary operation '>=' is not supported between string type and boolean type"
        "[line 17:5] Error at '>=': Binary operation '>=' is not supported between string type and number type"
    }

    tests! {
        greater_or_equal_non_literal in operator is ERR
        "[line 5:11] Error at '>=': Binary operation '>=' is not supported between number type and function type"
        "[line 6:11] Error at '>=': Binary operation '>=' is not supported between number type and class type"
        "[line 7:11] Error at '>=': Binary operation '>=' is not supported between number type and instance type"
        "[line 8:11] Error at '>=': Binary operation '>=' is not supported between number type and native function type"
    }

    tests! {
        less_literal in operator is ERR
        "[line 1:6] Error at '<': Binary operation '<' is not supported between boolean type and null type"
        "[line 2:6] Error at '<': Binary operation '<' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '<': Binary operation '<' is not supported between boolean type and string type"
        "[line 4:6] Error at '<': Binary operation '<' is not supported between boolean type and number type"
        "[line 6:6] Error at '<': Binary operation '<' is not supported between null type and null type"
        "[line 7:6] Error at '<': Binary operation '<' is not supported between null type and boolean type"
        "[line 8:6] Error at '<': Binary operation '<' is not supported between null type and string type"
        "[line 9:6] Error at '<': Binary operation '<' is not supported between null type and number type"
        "[line 11:5] Error at '<': Binary operation '<' is not supported between number type and null type"
        "[line 12:5] Error at '<': Binary operation '<' is not supported between number type and boolean type"
        "[line 13:5] Error at '<': Binary operation '<' is not supported between number type and string type"
        "[line 15:5] Error at '<': Binary operation '<' is not supported between string type and null type"
        "[line 16:5] Error at '<': Binary operation '<' is not supported between string type and boolean type"
        "[line 17:5] Error at '<': Binary operation '<' is not supported between string type and number type"
    }

    tests! {
        less_non_literal in operator is ERR
        "[line 5:11] Error at '<': Binary operation '<' is not supported between number type and function type"
        "[line 6:11] Error at '<': Binary operation '<' is not supported between number type and class type"
        "[line 7:11] Error at '<': Binary operation '<' is not supported between number type and instance type"
        "[line 8:11] Error at '<': Binary operation '<' is not supported between number type and native function type"
    }

    tests! {
        less_or_equal_literal in operator is ERR
        "[line 1:6] Error at '<=': Binary operation '<=' is not supported between boolean type and null type"
        "[line 2:6] Error at '<=': Binary operation '<=' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '<=': Binary operation '<=' is not supported between boolean type and string type"
        "[line 4:6] Error at '<=': Binary operation '<=' is not supported between boolean type and number type"
        "[line 6:6] Error at '<=': Binary operation '<=' is not supported between null type and null type"
        "[line 7:6] Error at '<=': Binary operation '<=' is not supported between null type and boolean type"
        "[line 8:6] Error at '<=': Binary operation '<=' is not supported between null type and string type"
        "[line 9:6] Error at '<=': Binary operation '<=' is not supported between null type and number type"
        "[line 11:5] Error at '<=': Binary operation '<=' is not supported between number type and null type"
        "[line 12:5] Error at '<=': Binary operation '<=' is not supported between number type and boolean type"
        "[line 13:5] Error at '<=': Binary operation '<=' is not supported between number type and string type"
        "[line 15:5] Error at '<=': Binary operation '<=' is not supported between string type and null type"
        "[line 16:5] Error at '<=': Binary operation '<=' is not supported between string type and boolean type"
        "[line 17:5] Error at '<=': Binary operation '<=' is not supported between string type and number type"
    }

    tests! {
        less_or_equal_non_literal in operator is ERR
        "[line 5:11] Error at '<=': Binary operation '<=' is not supported between number type and function type"
        "[line 6:11] Error at '<=': Binary operation '<=' is not supported between number type and class type"
        "[line 7:11] Error at '<=': Binary operation '<=' is not supported between number type and instance type"
        "[line 8:11] Error at '<=': Binary operation '<=' is not supported between number type and native function type"
    }

    tests! {
        multiply in operator is OK
        "15"
        "3.702"
        "0"
    }

    tests! {
        multiply_literal in operator is ERR
        "[line 1:6] Error at '*': Binary operation '*' is not supported between boolean type and null type"
        "[line 2:6] Error at '*': Binary operation '*' is not supported between boolean type and boolean type"
        "[line 3:6] Error at '*': Binary operation '*' is not supported between boolean type and string type"
        "[line 4:6] Error at '*': Binary operation '*' is not supported between boolean type and number type"
        "[line 6:6] Error at '*': Binary operation '*' is not supported between null type and null type"
        "[line 7:6] Error at '*': Binary operation '*' is not supported between null type and boolean type"
        "[line 8:6] Error at '*': Binary operation '*' is not supported between null type and string type"
        "[line 9:6] Error at '*': Binary operation '*' is not supported between null type and number type"
        "[line 11:5] Error at '*': Binary operation '*' is not supported between number type and null type"
        "[line 12:5] Error at '*': Binary operation '*' is not supported between number type and boolean type"
        "[line 13:5] Error at '*': Binary operation '*' is not supported between number type and string type"
        "[line 15:5] Error at '*': Binary operation '*' is not supported between string type and null type"
        "[line 16:5] Error at '*': Binary operation '*' is not supported between string type and boolean type"
        "[line 17:5] Error at '*': Binary operation '*' is not supported between string type and number type"
    }

    tests! {
        multiply_non_literal in operator is ERR
        "[line 5:11] Error at '*': Binary operation '*' is not supported between number type and function type"
        "[line 6:11] Error at '*': Binary operation '*' is not supported between number type and class type"
        "[line 7:11] Error at '*': Binary operation '*' is not supported between number type and instance type"
        "[line 8:11] Error at '*': Binary operation '*' is not supported between number type and native function type"
    }

    tests! {
        negate in operator is OK
        "-3"
        "3"
        "-3"
    }

    tests! {
        negate_literal in operator is ERR
        "[line 1:7] Error at '-': Unary operation '-' is not supported for string type"
        "[line 2:7] Error at '-': Unary operation '-' is not supported for null type"
        "[line 3:7] Error at '-': Unary operation '-' is not supported for boolean type"
    }

    tests! {
        negate_non_literal in operator is ERR
        "[line 5:7] Error at '-': Unary operation '-' is not supported for function type"
        "[line 6:7] Error at '-': Unary operation '-' is not supported for class type"
        "[line 7:7] Error at '-': Unary operation '-' is not supported for instance type"
        "[line 8:7] Error at '-': Unary operation '-' is not supported for native function type"
    }

    tests! {
        not in operator is OK
        "false"
        "true"
        "true"
        "true"
    }

    tests! {
        not_class in operator is ERR
        "[line 2:7] Error at '!': Unary operation '!' is not supported for class type"
        "[line 3:7] Error at '!': Unary operation '!' is not supported for instance type"
    }

    tests! {
        not_equals in operator is OK
        "false"
        "false"
        "true"
        "false"
        "true"
        "false"
        "true"
    }

    tests! {
        subtract in operator is OK
        "1"
        "0"
        "-4"
        "7"
    }
}
