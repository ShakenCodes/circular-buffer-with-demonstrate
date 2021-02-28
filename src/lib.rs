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
    pub fn empty(&self) -> bool {
        self.index == 0
    }
    pub fn full(&self) -> bool {
        self.capacity == self.index
    }
    pub fn get(&mut self) -> Result<i32, ()> {
        if self.index == 0 { return Err(()); }
        self.index -= 1;
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
        it "get yields an error" {
            assert_eq!(Err(()), b.get());
        }
        it "put yields an error" {
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
        it "get yields error" {
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
            it "is not empty" {
                assert_false!(b.empty());
            }
            it "is full" {
                assert_true!(b.full());
            }
            it "get retrives the item previously put" {
                assert_eq!(item, b.get().unwrap());
            }

            describe "and then get once, " {
                before {
                    let _ = b.get();
                }
                it "is empty" {
                    assert_true!(b.empty());
                }
                it "is not full" {
                    assert_false!(b.full());
                }
                it "get yields error" {
                    assert_eq!(Err(()), b.get());
                }
                it "put succeeds" {
                    assert_eq!(Ok(()), b.put(42));
                }
            }
        }
    }
    describe "given a buffer of capacity two" {       
        use super::*;
        before {
            #[allow(unused_mut)]
            let mut b = CircularBuffer::new(2);
        }
        it "is empty" {
            assert_true!(b.empty());
        }
        it "is not full" {
            assert_false!(b.full());
        }
        it "get yields error" {
            assert_eq!(Err(()), b.get());
        }
        it "put succeeds" {
            assert_eq!(Ok(()), b.put(42));
        }
        describe "when one item added" {
            before {
                #[allow(unused)]
                let first = 43;
                let _ = b.put(first);
            }
            it "is not empty" {
                assert_false!(b.empty());
            }
            it "is not full" {
                assert_false!(b.full());
            }
            it "get retrieve put value" {
                assert_eq!(Ok(first), b.get());
            }
            it "put succeeds" {
                assert_eq!(Ok(()), b.put(42));
            }
            }
    }
}
