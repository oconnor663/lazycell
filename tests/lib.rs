extern crate lazycell;

use lazycell::LazyCell;

#[test]
fn test_lazycell() {
    let lazycell = LazyCell::new();

    assert_eq!(lazycell.borrow(), None);
    assert!(!lazycell.filled());

    lazycell.fill(1).unwrap();

    assert!(lazycell.filled());
    assert_eq!(lazycell.borrow(), Some(&1));
    assert_eq!(lazycell.into_inner(), Some(1));
}

#[test]
fn test_already_filled_error() {
    let lazycell: LazyCell<usize> = LazyCell::new();

    lazycell.fill(1).unwrap();
    assert_eq!(lazycell.fill(1), Err(1));
}
