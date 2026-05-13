use naipes::core::Rng;

#[test]
fn zero_seed_is_substituted() {
    let mut a = Rng::new(0);
    let mut b = Rng::new(0xDEADBEEFCAFEBABE);
    assert_eq!(a.next_u64(), b.next_u64());
}

#[test]
fn five_samples_seed_1_are_distinct() {
    let mut rng = Rng::new(1);
    let mut seen = std::collections::HashSet::new();
    for _ in 0..5 {
        seen.insert(rng.next_u64());
    }
    assert_eq!(seen.len(), 5);
}

#[test]
fn bounded_in_range() {
    let mut rng = Rng::new(42);
    for _ in 0..1000 {
        let v = rng.bounded(40);
        assert!(v < 40);
    }
}

#[test]
fn shuffle_is_permutation() {
    let mut rng = Rng::new(7);
    let mut items: Vec<u32> = (0..40).collect();
    rng.shuffle(&mut items);
    let mut sorted = items.clone();
    sorted.sort();
    let expected: Vec<u32> = (0..40).collect();
    assert_eq!(sorted, expected);
}

#[test]
fn same_seed_same_shuffle() {
    let mut a: Vec<u32> = (0..40).collect();
    let mut b: Vec<u32> = (0..40).collect();
    Rng::new(123).shuffle(&mut a);
    Rng::new(123).shuffle(&mut b);
    assert_eq!(a, b);
}

#[test]
fn different_seed_different_shuffle() {
    let mut a: Vec<u32> = (0..40).collect();
    let mut b: Vec<u32> = (0..40).collect();
    Rng::new(1).shuffle(&mut a);
    Rng::new(2).shuffle(&mut b);
    assert_ne!(a, b);
}
