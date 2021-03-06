/*
 *   Copyright (c) 2021 Shaken Codes (Philip A. Markgraf)
 *   All rights reserved.
 */
pub struct CircularBuffer {
    capacity: usize,
    count: usize,
    input: usize,
    output: usize,
    values: Vec<i32>,
}
impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        Self{
            capacity,
            count: 0,
            input: 0,
            output: 0,
            values: vec![i32::MIN; capacity],
        }
    }
    pub fn empty(&self) -> bool {
        self.count == 0
    }
    pub fn full(&self) -> bool {
        self.capacity == self.count
    }
    pub fn get(&mut self) -> Result<i32, ()> {
        if self.empty() { return Err(()); }
        let v = self.values[self.output];
        self.output = CircularBuffer::increment_and_clip(self.output, self.capacity);
        self.count -= 1;
        Ok(v)
     }
    pub fn put(&mut self, v: i32) -> Result<(), ()> {
        if self.full() { return Err(()); }
        self.values[self.input] = v;
        self.input = CircularBuffer::increment_and_clip(self.input, self.capacity);
        self.count += 1;
        Ok(())
    }
    fn increment_and_clip(n: usize, c: usize) -> usize {
        if (n + 1) < c { return n + 1; }
        0
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
            describe "when second item added" {
                before {
                    #[allow(unused)]
                    let second = -11;
                    let _ = b.put(second);
                }
                it "is not empty" {
                    assert_false!(b.empty());
                }
                it "is full" {
                    assert_true!(b.full());
                }
                it "get retrieve put values" {
                    assert_eq!(Ok(first), b.get());
                    assert_eq!(Ok(second), b.get());
                }
                it "put yields an error" {
                    assert_eq!(Err(()), b.put(42));
                }        
                it "retrieve once, add two, retrieve remaining values" {
                    assert_eq!(Ok(first), b.get());
                    let third = 10101;
                    assert_eq!(Ok(()), b.put(third));
                    assert_eq!(Ok(second), b.get());
                    assert_eq!(Ok(third), b.get());
                }
            }
        }
    }
}
