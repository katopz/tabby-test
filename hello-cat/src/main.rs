use std::num::Saturating;

// An example about Saturating MIN in Rust
fn main() {
    assert_eq!(<Saturating<usize>>::MIN, Saturating(usize::MIN));
}
