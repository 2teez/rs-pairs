#[test]
fn generate_pairs() {
    let pairs: Pairs<&str, u32> = Pairs::new("tim", 23);
    assert_eq!(
        pairs,
        Pairs {
            first: "tim",
            second: 23
        }
    );
}

#[test]
#[should_panic(expected = "tolu")]
fn get_second() {
    let pairs = Pairs::new(1, "kunle");
    assert_eq!(pairs.second, "tolu");
}

#[test]
fn get_first_value() {
    let pairs = pairs::Pairs::<_>::new("temiloluwa", 11);
    assert_eq!(pairs.first, "temiloluwa");
    assert_eq!(pairs.second, 11);
}
