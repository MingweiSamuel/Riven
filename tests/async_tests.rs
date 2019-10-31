/// This is just a huge hack to make a test runner (with no test cases)
/// look as if it's running a bunch of (async) test cases.
#[macro_export]
macro_rules! async_tests {
    ( $runner:ident { $( $name:ident : async $eval:block, )* } ) => {
        fn $runner(_: &[()]) {
            const TUPLE_OK: (u32, u32) = (1, 0);
            const TUPLE_ERR: (u32, u32) = (0, 1);

            std::process::exit({
                let mut rt = tokio::runtime::current_thread::Runtime::new()
                    .expect("Failed to create runtime.");

                let (_, errs) = rt.block_on(async {
                    println!();
                    println!("running tests");
                    println!();
                    let mut oks: u32 = 0;
                    let mut errs: u32 = 0;
                    $(
                        let $name = async {
                            let result: Result<(), String> = async {
                                $eval
                            }.await;
                            print!("test {} ... ", stringify!($name));
                            match &result {
                                Ok(_) => {
                                    println!("{}", "ok".green());
                                    TUPLE_OK
                                }
                                Err(msg) => {
                                    println!("{}", "error".bright_red());
                                    println!("{}", msg);
                                    TUPLE_ERR
                                }
                            }
                        };
                    )*
                    $(
                        let $name = $name.await;
                        oks += $name.0; errs += $name.1;
                    )*
                    println!();
                    print!("test result: {}. ", if errs > 0 { "error".bright_red() } else { "ok".green() });
                    println!("{} passed; {} failed; 0 ignored; 0 measured; 0 filtered out", oks, errs);
                    println!();
                    (oks, errs)
                });
                // Just returns #errs as exit code.
                errs as i32
            });
        }
    };
}

#[macro_export]
macro_rules! rassert {
    ( $x:expr ) => {
        {
            if $x { Ok(()) } else { Err(stringify!($x)) }?
        }
    };
    ( $x:expr, $format:expr $(, $arg:expr)* ) => {
        {
            if $x { Ok(()) } else { Err( format!($format, $( $arg )* ) ) }?
        }
    };
}

#[macro_export]
macro_rules! rassert_eq {
    ( $a:expr, $b:expr ) => { rassert!($a == $b) };
    ( $a:expr, $b:expr, $format:expr $(, $arg:expr)* ) => {
        rassert!($a == $b, $format $(, $arg )* )
    };
}

#[macro_export]
macro_rules! rassert_ne {
    ( $a:expr, $b:expr ) => { rassert!($a != $b) };
    ( $a:expr, $b:expr, $format:expr $(, $arg:expr)* ) => {
        rassert!($a != $b, $format $(, $arg )* )
    };
}
