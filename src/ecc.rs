use crate::checksumm;
use reed_solomon;

pub fn get_ecc_size(size: usize) -> usize {
    0
}

pub fn get_wrapped_size(size: usize) -> usize {
    get_ecc_size(size) + size
}

pub fn get_unwrapped_size(size: usize) -> Option<usize> {
    None
}

pub fn wrap(msg: &[u8]) -> Vec<u8> {
    Vec::new()
}

pub fn unwrap_raw(wrapped: &[u8]) -> Result<Vec<u8>, ()> {
    Err(())
}

// TODO Add custom error type
pub fn unwrap(wrapped: &[u8], max_steps: usize) -> Result<Vec<u8>, ()> {
    Err(())
}

