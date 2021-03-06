extern crate roaring;
use roaring::RoaringBitmap;

#[test]
fn array_not() {
    let sup = (0..2000).collect::<RoaringBitmap>();
    let sub = (1000..3000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn array() {
    let sup = (0..4000).collect::<RoaringBitmap>();
    let sub = (2000..3000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), true);
}

#[test]
fn array_bitmap_not() {
    let sup = (0..2000).collect::<RoaringBitmap>();
    let sub = (1000..15000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn bitmap_not() {
    let sup = (0..6000).collect::<RoaringBitmap>();
    let sub = (4000..10000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn bitmap() {
    let sup = (0..20000).collect::<RoaringBitmap>();
    let sub = (5000..15000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), true);
}

#[test]
fn bitmap_array_not() {
    let sup = (0..20000).collect::<RoaringBitmap>();
    let sub = (19000..21000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn bitmap_array() {
    let sup = (0..20000).collect::<RoaringBitmap>();
    let sub = (18000..20000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), true);
}

#[test]
fn arrays_not() {
    let sup = (0..2000)
        .chain(1_000_000..1_002_000)
        .collect::<RoaringBitmap>();
    let sub = (100_000..102_000)
        .chain(1_100_000..1_102_000)
        .collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn arrays() {
    let sup = (0..3000).chain(100_000..103_000).collect::<RoaringBitmap>();
    let sub = (0..2000).chain(100_000..102_000).collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), true);
}

#[test]
fn bitmaps_not() {
    let sup = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_010_000)
        .collect::<RoaringBitmap>();
    let sub = (100_000..106_000)
        .chain(1_100_000..1_106_000)
        .collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), false);
}

#[test]
fn bitmaps() {
    let sup = (0..1_000_000)
        .chain(2_000_000..2_010_000)
        .collect::<RoaringBitmap>();
    let sub = (0..10_000)
        .chain(500_000..510_000)
        .collect::<RoaringBitmap>();
    assert_eq!(sub.is_subset(&sup), true);
}
