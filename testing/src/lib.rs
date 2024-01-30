pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(str: String) -> String {
    format!("Hello {}", str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failing_assert() {
        assert_eq!(2, 3);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Test pass as panic used as above");
    }

    #[test]
    #[should_panic(expected = "fail for this custom error")]
    fn test_panic_with_expected_custom_error() {
        panic!("fail for this custom error");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn greeting_contains_name() {
        let result1 = greeting(String::from("john"));
        let result2 = greeting(String::from("doe"));

        assert!( // pass
            result1.contains("john"),
            "The resulting string don't contains john"
        );
        assert!( // fails by custom error defined here
            result2.contains("john"),
            "The resulting string don't contains john"
        );
    }
}


/*
    cargo test compiles your code in test mode and runs the resulting binary. By Default
    all tests run in parallel in separate thread.
    some cargo commands for resulting test binary

    cargo test -- --test-threads=1 :- use a single thread to run tests serially
    cargo test -- --show-output :- shows outputs of passing tests also
    cargo test [test_name] :- for running the subset of test
    cargo test -- --ignored :- run only the ignored tests

*/
