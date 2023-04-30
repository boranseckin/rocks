#[macro_export]
macro_rules! tests {
    ($file:ident in $scope:ident is OK $($expected:expr)+) => {
        #[test]
        fn $file() {
            use assert_cmd::Command;
            // output concats expected with new line
            let output = vec![$($expected),+].join("\n");
            let header = format!("rocks v{}", env!("CARGO_PKG_VERSION"));
            let file = format!("examples/{}/{}.rocks", stringify!($scope), stringify!($file));

            Command::cargo_bin("rocks").unwrap()
                .arg(file)
                .assert()
                .stdout(format!("{header}\n{output}\n"))
                .success();
        }
    };

    ($file:ident in $scope:ident is ERR $($expected:expr)+) => {
        #[test]
        fn $file() {
            use assert_cmd::Command;
            // output concats expected with new line
            let output = vec![$($expected),+].join("\n");
            let file = format!("examples/{}/{}.rocks", stringify!($scope), stringify!($file));

            Command::cargo_bin("rocks").unwrap()
                .arg(file)
                .assert()
                .stderr(format!("{output}\n"))
                .failure();
        }
    };
}
