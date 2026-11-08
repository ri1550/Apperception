use apperception_api::run;

#[test]
fn run_delegates_to_greeting() {
    assert_eq!(run(), "Hello, world!");
}
