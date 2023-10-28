## Writing Automated Tests

We can write tests that assert, for example, that when we pass 3 to the add_two function, the returned value is 5.
We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill: although we can’t cover every detail about how to write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing facilities.
We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options provided for running your tests, and how to organize tests into unit tests and integration tests.

#### Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator.

        --include-ignored
                        Run ignored and not ignored tests
        --ignored       Run only ignored tests
        --force-run-in-process
                        Forces tests to run in-process when panic=abort
        --exclude-should-panic
                        Excludes tests marked as should_panic
        --test          Run tests and not benchmarks
        --bench         Run benchmarks instead of tests
        --list          List all tests and benchmarks
    -h, --help          Display this message
        --logfile PATH  Write logs to the specified file
        --nocapture     don't capture stdout/stderr of each task, allow
                        printing directly
        --test-threads n_threads
                        Number of threads used for running tests in parallel
        --skip FILTER   Skip tests whose names contain FILTER (this flag can
                        be used multiple times)
    -q, --quiet         Display one character per test instead of one line.
                        Alias to --format=terse
        --exact         Exactly match filters rather than by substring
        --color auto|always|never
                        Configure coloring of output:
                        auto = colorize if stdout is a tty and tests are run
                        on serially (default);
                        always = always colorize output;
                        never = never colorize output;
        --format pretty|terse|json|junit
                        Configure formatting of output:
                        pretty = Print verbose output;
                        terse = Display one character per test;
                        json = Output a json document;
                        junit = Output a JUnit document
        --show-output   Show captured stdout of successful tests
    -Z unstable-options Enable nightly-only flags:
                        unstable-options = Allow use of experimental features
        --report-time   Show execution time of each test.
                        Threshold values for colorized output can be
                        configured via
                        `RUST_TEST_TIME_UNIT`, `RUST_TEST_TIME_INTEGRATION`
                        and
                        `RUST_TEST_TIME_DOCTEST` environment variables.
                        Expected format of environment variable is
                        `VARIABLE=WARN_TIME,CRITICAL_TIME`.
                        Durations must be specified in milliseconds, e.g.
                        `500,2000` means that the warn time
                        is 0.5 seconds, and the critical time is 2 seconds.
                        Not available for --format=terse
        --ensure-time   Treat excess of the test execution time limit as
                        error.
                        Threshold values for this option can be configured via
                        `RUST_TEST_TIME_UNIT`, `RUST_TEST_TIME_INTEGRATION`
                        and
                        `RUST_TEST_TIME_DOCTEST` environment variables.
                        Expected format of environment variable is
                        `VARIABLE=WARN_TIME,CRITICAL_TIME`.
                        `CRITICAL_TIME` here means the limit that should not
                        be exceeded by test.
        --shuffle       Run tests in random order
        --shuffle-seed SEED
                        Run tests in random order; seed the random number
                        generator with SEED

#### Test Attributes:

    `#[test]`        - Indicates a function is a test to be run. This function
                       takes no arguments.
    `#[bench]`       - Indicates a function is a benchmark to be run. This
                       function takes one argument (test::Bencher).
    `#[should_panic]` - This function (also labeled with `#[test]`) will only pass if
                        the code causes a panic (an assertion failure or panic!)
                        A message may be provided, which the failure string must
                        contain: #[should_panic(expected = "foo")].
    `#[ignore]`       - When applied to a function which is already attributed as a
                        test, then the test runner will ignore these tests during
                        normal test runs. Running with --ignored or --include-ignored will run
                        these tests.

#### Integration Tests for Binary Crates

If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with use to make the important functionality available. If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
