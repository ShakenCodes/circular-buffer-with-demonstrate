/*
 *   Copyright (c) 2021 
 *   All rights reserved.
 */
pub struct CircularBuffer {

}
impl CircularBuffer {
    pub fn new(_: usize) -> Self { Self{} }
    pub fn empty(&self) -> bool { true }
    pub fn full(&self) -> bool { true }
}

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
mod tests {
    use super::*;
    speculate! {
        describe "given a buffer of capacity zero" {
            before {
                let buf = CircularBuffer::new(0);
            }
            it "is empty" {
                assert_eq!(true, buf.empty());
            }
            it "is full" {
                assert_eq!(true, buf.full());
            }
        }
    }
    use speculate::speculate;
}
