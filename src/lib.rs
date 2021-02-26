/*
 *   Copyright (c) 2021 Shaken Codes (Philip A. Markgraf)
 *   All rights reserved.
 */
pub struct CircularBuffer {

}
impl CircularBuffer {
    pub fn new(_: usize) -> Self { Self{} }
    pub fn empty(&self) -> bool { true }
    pub fn full(&self) -> bool { true }
    pub fn put(&self, _: i32) -> bool { false }
}

#[cfg(test)]
use all_asserts::*;

use demonstrate::demonstrate;

demonstrate! {
    describe "given a buffer of capacity zero" {       
        use super::*;
        before {
            let b = CircularBuffer::new(0);
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
    }
}
