// src/lib.rs

pub mod structures;
pub mod decoder;
pub mod archive;
pub mod encoder;
pub mod block;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
