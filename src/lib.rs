/*
 *   Copyright (c) 2021 Shaken Codes (Philip A. Markgraf)
 *   All rights reserved.
 */
pub struct CircularBuffer {
}
impl CircularBuffer {
    pub fn new(_: usize) -> Self { Self{ } }
    pub fn empty(&self) -> bool { true }
    pub fn full(&self) -> bool { true }
    pub fn put(&self, _: i32) -> Result<(), ()> { Err(()) }
    pub fn get(&self) -> Result<i32, ()> { Err(()) }
}

use demonstrate::demonstrate;
#[cfg(test)]
use all_asserts::*;

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
            assert_eq!(Err(()), b.put(42));
        }
        it "get fails with pre-set bad value" {
            assert_eq!(Err(()), b.get());
        }

        describe ", after put attempt" {
            before {
                assert_eq!(Err(()), b.put(-1));
            }

            it "is empty" {
                assert_true!(b.empty());
            }
            it "is full" {
                assert_true!(b.full());
            }
            it "put fails with false" {
                assert_eq!(Err(()), b.put(42));
            }
            it "get fails with pre-set bad value" {
                assert_eq!(Err(()), b.get());
            }
        }
    }
}
