use super::*;

const INPUT: &str = "\
\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[0m\u{1b}[1m: unused import: `super::*`\u{1b}[0m
\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m--> \u{1b}[0m\u{1b}[0msrc/lib.rs:3:9\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m3\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\
\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m    use super::*;\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m| \u{1b}[0m\u{1b}[0m        \u{1b}[0m\u{1b}[0m\
\u{1b}[1m\u{1b}[33m^^^^^^^^\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m= \u{1b}[0m\u{1b}[0m\u{1b}[1mnote\u{1b}[0m\
\u{1b}[0m: `#[warn(unused_imports)]` on by default\u{1b}[0m

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m:\u{1b}[0m `termwrap` (lib test) \
generated 1 warning (run `cargo fix --lib -p termwrap --tests` to apply 1 suggestion)
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m    Finished\u{1b}[0m test [unoptimized + debuginfo] \
target(s) in 0.00s
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m     Running\u{1b}[0m unittests src/lib.rs (target/debug/deps\
/termwrap-bc958128818bd908)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m   Doc-tests\u{1b}[0m termwrap

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

";

#[test]
fn width_0() {
    assert_eq!(termwrap(INPUT, 0, "\\"), INPUT);
}

#[test]
fn width_120() {
    assert_eq!(termwrap(INPUT, 120, "\\"), INPUT);
}

#[test]
fn width_50() {
    assert_eq!(
        termwrap(INPUT, 50, "\\"),
        "\
\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[0m\u{1b}[1m: unused import: `super::*`\u{1b}[0m
\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m--> \u{1b}[0m\u{1b}[0msrc/lib.rs:3:9\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m3\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\
\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m    use super::*;\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m| \u{1b}[0m\u{1b}[0m        \u{1b}[0m\u{1b}[0m\
\u{1b}[1m\u{1b}[33m^^^^^^^^\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m= \u{1b}[0m\u{1b}[0m\u{1b}[1mnote\u{1b}[0m\
\u{1b}[0m: `#[warn(unused_imports)]` on by default\\
\u{1b}[0m

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m:\u{1b}[0m `termwrap` (lib test) \
generated 1 warnin\\
g (run `cargo fix --lib -p termwrap --tests` to a\\
pply 1 suggestion)
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m    Finished\u{1b}[0m test [unoptimized + debuginfo] targe\\
t(s) in 0.00s
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m     Running\u{1b}[0m unittests src/lib.rs (target/debug/d\\
eps/termwrap-bc958128818bd908)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0\\
 measured; 0 filtered out; finished in 0.00s

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m   Doc-tests\u{1b}[0m termwrap

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0\\
 measured; 0 filtered out; finished in 0.00s

\
        ",
    );
}

#[test]
fn width_66() {
    assert_eq!(
        termwrap(INPUT, 66, "\\"),
        "\
\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[0m\u{1b}[1m: unused import: `super::*`\u{1b}[0m
\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m--> \u{1b}[0m\u{1b}[0msrc/lib.rs:3:9\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m3\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\
\u{1b}[0m\u{1b}[0m \u{1b}[0m\u{1b}[0m    use super::*;\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m| \u{1b}[0m\u{1b}[0m        \u{1b}[0m\u{1b}[0m\
\u{1b}[1m\u{1b}[33m^^^^^^^^\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m|\u{1b}[0m
\u{1b}[0m  \u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[38;5;12m= \u{1b}[0m\u{1b}[0m\u{1b}[1mnote\u{1b}[0m\
\u{1b}[0m: `#[warn(unused_imports)]` on by default\u{1b}[0m

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[33mwarning\u{1b}[0m\u{1b}[1m:\u{1b}[0m `termwrap` (lib test) \
generated 1 warning (run `cargo fi\\
x --lib -p termwrap --tests` to apply 1 suggestion)
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m    Finished\u{1b}[0m test [unoptimized + debuginfo] \
target(s) in 0.00s
\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m     Running\u{1b}[0m unittests src/lib.rs (target/debug/deps\
/termwrap-bc9\\
58128818bd908)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 fil\\
tered out; finished in 0.00s

\u{1b}[0m\u{1b}[0m\u{1b}[1m\u{1b}[32m   Doc-tests\u{1b}[0m termwrap

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 fil\\
tered out; finished in 0.00s

\
        ",
    );
}
