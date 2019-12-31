/// This is just a huge hack to make a test runner (with no test cases)
/// look as if it's running a bunch of (async) test cases.
#[macro_export]
macro_rules! async_tests {
    ( $runner:ident { $( $name:ident : async $eval:block, )* } ) => {
        fn $runner(_: &[()]) {
            env_logger::init();

            std::process::exit({
                let mut rt = tokio::runtime::Runtime::new()
                    .expect("Failed to create runtime.");

                let (_, errs) = rt.block_on(async {
                    println!();
                    println!("running tests");
                    println!();
                    let mut oks: u32 = 0;
                    let mut errs: u32 = 0;
                    $(
                        let $name = async {
                            let result: std::result::Result<(), String> = async {
                                $eval
                            }.await;
                            result
                        };
                        // let $name = tokio_executor::Executor::spawn_with_handle(
                        //     &mut tokio_executor::DefaultExecutor::current(), $name)
                        //     .expect("Failed to spawn.");
                        let $name = tokio::spawn($name);
                    )*
                    $(
                        let $name = $name.await;
                    )*
                    $(
                        print!("test {} ... ", stringify!($name));
                        match $name {
                            Ok(_) => {
                                println!("{}", "ok".green());
                                oks += 1;
                            }
                            Err(msg) => {
                                println!("{}", "error".bright_red());
                                println!("{}", msg);
                                errs += 1;
                            }
                        }
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
