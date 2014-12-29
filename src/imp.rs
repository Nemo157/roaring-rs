use std::{ u16 };
use std::slice::BinarySearchResult::{ Found, NotFound };

use iter::RoaringIterator;
use container::Container;

pub struct RoaringBitmap {
    containers: Vec<Container>,
}

type RB = RoaringBitmap;

#[inline]
pub fn new() -> RB {
    RB { containers: Vec::new() }
}

pub fn insert(this: &mut RB, value: u32) -> bool {
    let (key, index) = calc_loc(value);
    let container = match this.containers.as_slice().binary_search(|container| key.cmp(&container.key())) {
        Found(loc) => &mut this.containers[loc],
        NotFound(loc) => {
            this.containers.insert(loc, Container::new(key));
            &mut this.containers[loc]
        },
    };
    container.insert(index)
}

pub fn remove(this: &mut RB, value: u32) -> bool {
    let (key, index) = calc_loc(value);
    match this.containers.as_slice().binary_search(|container| key.cmp(&container.key())) {
        Found(loc) => {
            if this.containers[loc].remove(index) {
                if this.containers[loc].len() == 0 {
                    this.containers.remove(loc);
                }
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn contains(this: &RB, value: u32) -> bool {
    let (key, index) = calc_loc(value);
    match this.containers.as_slice().binary_search(|container| key.cmp(&container.key())) {
        Found(loc) => this.containers[loc].contains(index),
        NotFound(_) => false,
    }
}

#[inline]
pub fn clear(this: &mut RB) {
    this.containers.clear();
}

#[inline]
pub fn is_empty(this: &RB) -> bool {
    this.containers.is_empty()
}

pub fn len(this: &RB) -> uint {
    this.containers
        .iter()
        .map(|container| container.len() as uint)
        .fold(0, |sum, len| sum + len)
}

#[inline]
pub fn iter<'a>(this: &'a RB) -> RoaringIterator<'a> {
    RoaringIterator::new(box this.containers.iter())
}

pub fn is_disjoint(this: &RB, other: &RB) -> bool {
    let (mut i1, mut i2) = (this.containers.iter(), other.containers.iter());
    let (mut c1, mut c2) = (i1.next(), i2.next());
    loop {
        match (c1.map(|c| c.key()), c2.map(|c| c.key())) {
            (None, _) | (_, None) => return true,
            (key1, key2) if key1 == key2 => {
                if c1.unwrap().is_disjoint(c2.unwrap()) {
                    c1 = i1.next();
                    c2 = i2.next();
                } else {
                    return false;
                }
            },
            (key1, key2) if key1 < key2 => c1 = i1.next(),
            (key1, key2) if key1 > key2 => c2 = i2.next(),
            (_, _) => panic!(),
        }
    }
}

pub fn is_subset(this: &RB, other: &RB) -> bool {
    let (mut i1, mut i2) = (this.containers.iter(), other.containers.iter());
    let (mut c1, mut c2) = (i1.next(), i2.next());
    loop {
        match (c1.map(|c| c.key()), c2.map(|c| c.key())) {
            (None, _) => return true,
            (_, None) => return false,
            (key1, key2) if key1 == key2 => {
                if c1.unwrap().is_subset(c2.unwrap()) {
                    c1 = i1.next();
                    c2 = i2.next();
                } else {
                    return false;
                }
            },
            (key1, key2) if key1 < key2 => return false,
            (key1, key2) if key1 > key2 => c2 = i2.next(),
            (_, _) => panic!(),
        }
    }
}

#[inline]
pub fn is_superset(this: &RB, other: &RB) -> bool {
    other.is_subset(this)
}

#[inline]
pub fn from_iter<I: Iterator<u32>>(iterator: I) -> RB {
    let mut rb = new();
    rb.extend(iterator);
    rb
}

#[inline]
pub fn extend<I: Iterator<u32>>(this: &mut RB, mut iterator: I) {
    for value in iterator {
        this.insert(value);
    }
}

#[inline]
fn calc_loc(index: u32) -> (u16, u16) { ((index >> u16::BITS) as u16, index as u16) }

#[cfg(test)]
mod test {
    use std::{ u16, u32 };
    use super::{ calc_loc };

    #[test]
    fn test_calc_location() {
        assert_eq!((0, 0), calc_loc(0));
        assert_eq!((0, 1), calc_loc(1));
        assert_eq!((0, u16::MAX - 1), calc_loc(u16::MAX as u32 - 1));
        assert_eq!((0, u16::MAX), calc_loc(u16::MAX as u32));
        assert_eq!((1, 0), calc_loc(u16::MAX as u32 + 1));
        assert_eq!((1, 1), calc_loc(u16::MAX as u32 + 2));
        assert_eq!((u16::MAX, u16::MAX - 1), calc_loc(u32::MAX - 1));
        assert_eq!((u16::MAX, u16::MAX), calc_loc(u32::MAX));
    }
}
