#[macro_use]
mod common;

#[cfg(test)]
mod field {
    tests! {
        call_function_field in field is OK
        "bar"
        "1"
        "2"
    }

    tests! {
        call_nonfunction_field in field is ERR
        "[line 6:9] Error at ')': Can only call functions and classes"
    }

    tests! {
        get_and_set_method in field is OK
        "other"
        "1"
        "method"
        "2"
    }

    tests! {
        get_on_bool in field is ERR
        "[line 1:6] Error at 'foo': Only instances have properties"
    }

    tests! {
        get_on_class in field is ERR
        "[line 2:5] Error at 'bar': Only instances have properties"
    }

    tests! {
        get_on_function in field is ERR
        "[line 2:5] Error at 'bar': Only instances have properties"
    }

    tests! {
        get_on_null in field is ERR
        "[line 1:6] Error at 'foo': Only instances have properties"
    }

    tests! {
        get_on_num in field is ERR
        "[line 1:1] Error: Unterminated number"
    }

    tests! {
        get_on_string in field is ERR
        "[line 1:7] Error at 'foo': Only instances have properties"
    }

    tests! {
        many in field is OK
        "apple"
        "apricot"
        "avocado"
        "banana"
        "bilberry"
        "blackberry"
        "blackcurrant"
        "blueberry"
        "boysenberry"
        "cantaloupe"
        "cherimoya"
        "cherry"
        "clementine"
        "cloudberry"
        "coconut"
        "cranberry"
        "currant"
        "damson"
        "date"
        "dragonfruit"
        "durian"
        "elderberry"
        "feijoa"
        "fig"
        "gooseberry"
        "grape"
        "grapefruit"
        "guava"
        "honeydew"
        "huckleberry"
        "jabuticaba"
        "jackfruit"
        "jambul"
        "jujube"
        "juniper"
        "kiwifruit"
        "kumquat"
        "lemon"
        "lime"
        "longan"
        "loquat"
        "lychee"
        "mandarine"
        "mango"
        "marionberry"
        "melon"
        "miracle"
        "mulberry"
        "nance"
        "nectarine"
        "olive"
        "orange"
        "papaya"
        "passionfruit"
        "peach"
        "pear"
        "persimmon"
        "physalis"
        "pineapple"
        "plantain"
        "plum"
        "plumcot"
        "pomegranate"
        "pomelo"
        "quince"
        "raisin"
        "rambutan"
        "raspberry"
        "redcurrant"
        "salak"
        "salmonberry"
        "satsuma"
        "strawberry"
        "tamarillo"
        "tamarind"
        "tangerine"
        "tomato"
        "watermelon"
        "yuzu"
    }

    tests! {
        method in field is OK
        "got method"
        "arg"
    }

    tests! {
        method_binds_this in field is OK
        "foo1"
        "1"
    }

    tests! {
        on_instance in field is OK
        "bar value"
        "baz value"
        "bar value"
        "baz value"
    }

    tests! {
        set_evaluation_order in field is ERR
        "[line 1:1] Error at 'undefined1': Undefined variable 'undefined1'"
    }

    tests! {
        set_on_bool in field is ERR
        "[line 1:6] Error at 'foo': Only instances can have fields"
    }

    tests! {
        set_on_class in field is ERR
        "[line 2:5] Error at 'bar': Only instances can have fields"
    }

    tests! {
        set_on_function in field is ERR
        "[line 2:5] Error at 'bar': Only instances can have fields"
    }

    tests! {
        set_on_null in field is ERR
        "[line 1:6] Error at 'foo': Only instances can have fields"
    }

    tests! {
        set_on_num in field is ERR
        "[line 1:1] Error: Unterminated number"
    }

    tests! {
        set_on_string in field is ERR
        "[line 1:7] Error at 'foo': Only instances can have fields"
    }

    tests! {
        undefined in field is ERR
        "[line 4:5] Error at 'bar': Undefined property 'bar'"
    }
}

