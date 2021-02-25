/*
 *   Copyright (c) 2021 
 *   All rights reserved.
 */
pub struct CircularBuffer {

}
impl CircularBuffer {
    pub fn new(_: usize) -> Self { Self{} }
    pub fn is_empty(&self) -> bool { true }
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
            it "when created, is_empty() is true" {
                assert_eq!(true, buf.is_empty());
            }
        }
    }
    use speculate::speculate;
}
