/*
 *   Copyright (c) 2021 Shaken Codes (Philip A. Markgraf)
 *   All rights reserved.
 */
pub struct CircularBuffer {
    empty_buffer_value: i32,
}
impl CircularBuffer {
    pub fn new(_: usize, empty_buffer_value: i32) -> Self { Self{ empty_buffer_value } }
    pub fn empty(&self) -> bool { true }
    pub fn full(&self) -> bool { true }
    pub fn put(&self, _: i32) -> bool { false }
    pub fn get(&self) -> i32 { self.empty_buffer_value }
}

use demonstrate::demonstrate;
#[cfg(test)]
use all_asserts::*;

demonstrate! {
    describe "given a buffer of capacity zero" {       
        use super::*;
        before {
            let bad = -13579;
            let b = CircularBuffer::new(0, bad);
        }
        it "is empty" {
            assert_true!(b.empty());
        }
        it "is full" {
            assert_true!(b.full());
        }
        it "put fails with false" {
            assert_false!(b.put(42));
        }
        it "get fails with pre-set bad value" {
            assert_eq!(bad, b.get());
        }
    }
}
