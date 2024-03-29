#[macro_export]
macro_rules! tests {
    ($file:ident in $scope:ident is OK $($expected:expr)*) => {
        #[test]
        fn $file() {
            use rocks_lang::rocks;

            let mut expected = vec![$($expected),*];

            // if exists, concat expected lines with new lines
            let expected = match expected.len() {
                0 => "".to_string(),
                _ => {
                    // add empty string to end to add new line to end of output
                    expected.push("");
                    expected.join("\n")
                }
            };

            let mut output = Vec::new();
            let mut rocks = rocks::new(&mut output);

            rocks.run_file(format!("tests/target/{}/{}.rocks", stringify!($scope), stringify!($file)));

            // drop rocks here to avoid compiler's drop check error before accessing output
            drop(rocks);
            assert_eq!(expected, std::str::from_utf8(&output).unwrap());
        }
    };

    ($file:ident in $scope:ident is ERR $($expected:expr)+) => {
        #[test]
        fn $file() {
            use assert_cmd::Command;
            // output concats expected with new line
            let output = vec![$($expected),+].join("\n");
            let file = format!("tests/target/{}/{}.rocks", stringify!($scope), stringify!($file));

            Command::cargo_bin("rocks").unwrap()
                .arg(file)
                .assert()
                .stderr(format!("{output}\n"))
                .failure();
        }
    };
}
