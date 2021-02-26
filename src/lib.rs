/*
 *   Copyright (c) 2021 Shaken Codes (Philip A. Markgraf)
 *   All rights reserved.
 */
pub struct CircularBuffer {
    capacity: usize,
    index: usize,
    value: i32,
}
impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self{
            capacity,
            index: 0,
            value: i32::MIN
        }
    }
    pub fn empty(&self) -> bool { true }
    pub fn full(&self) -> bool { self.capacity == 0 }
    pub fn get(&self) -> Result<i32, ()> {
        if self.index == 0 { return Err(()); }
        Ok(self.value)
     }
    pub fn put(&mut self, v: i32) -> Result<(), ()> {
        if self.full() { return Err(()); }
        self.value = v;
        self.index += 1;
        Ok(())
    }
}

use demonstrate::demonstrate;
#[cfg(test)]
use all_asserts::*;

demonstrate! {
    describe "given a buffer of capacity zero" {       
        use super::*;
        before {
            #[allow(unused_mut)]
            let mut b = CircularBuffer::new(0);
        }
        it "is empty" {
            assert_true!(b.empty());
        }
        it "is full" {
            assert_true!(b.full());
        }
        it "get fails with pre-set bad value" {
            assert_eq!(Err(()), b.get());
        }
        it "put fails with false" {
            assert_eq!(Err(()), b.put(42));
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
            it "get fails with pre-set bad value" {
                assert_eq!(Err(()), b.get());
            }
            it "put fails with false" {
                assert_eq!(Err(()), b.put(42));
            }
        }
    }
    describe "given a buffer of capacity one" {       
        use super::*;
        before {
            #[allow(unused_mut)]
            let mut b = CircularBuffer::new(1);
        }
        it "is empty" {
            assert_true!(b.empty());
        }
        it "is not full" {
            assert_false!(b.full());
        }
        it "get fails with pre-set bad value" {
            assert_eq!(Err(()), b.get());
        }
        it "put succeeds" {
            assert_eq!(Ok(()), b.put(42));
        }

        describe "and one item put," {
            before {
                let item = 111;
                assert_eq!(Ok(()), b.put(item));
            }
            it "get retrives the item previously put" {
                assert_eq!(item, b.get().unwrap());
            }
        }
    }
}
