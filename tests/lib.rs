extern crate lazycell;

use lazycell::LazyCell;

#[test]
fn test_lazycell() {
    let lazycell = LazyCell::new();

    assert_eq!(lazycell.borrow(), None);
    assert!(!lazycell.filled());

    lazycell.fill(1);

    assert!(lazycell.filled());
    assert_eq!(lazycell.borrow(), Some(&1));
    assert_eq!(lazycell.into_inner(), Some(1));
}

#[test]
#[should_panic(expected = "lazy cell is already filled")]
fn test_already_filled_panic() {
    let lazycell: LazyCell<usize> = LazyCell::new();

    lazycell.fill(1);
    lazycell.fill(1);
}
