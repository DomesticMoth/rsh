use crate::checksumm;
use reed_solomon;
use std::usize;

struct AddSubCombinations {
    left: usize,
    right: usize,
}

impl AddSubCombinations {
    fn new() -> Self {
        Self{
            left: 0,
            right: 0,
        }
    }
}

impl Iterator for AddSubCombinations {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == usize::MAX || self.right == usize::MAX {
            return None;
        }
        let ret = Some(
            (self.left, self.right)
        );
        if self.left == self.right {
            self.left += 1;
            self.right = 0;
        } else {
            if self.left < self.right {
                self.left += 1;
            }
            let tmp = self.left;
            self.left = self.right;
            self.right = tmp;
        }
        ret
    }
}

struct CombinationsGenerator {
    mask: Vec<u8>,
}


impl CombinationsGenerator {
    fn new(len: usize, add: usize, sub: usize) -> Option<Self> {
        if add + sub < len {
            let mut mask = Vec::with_capacity(len);;
            for _ in 0..add {
                mask.push(1);
            }
            for _ in 0..(len-(add+sub)) {
                mask.push(0);
            }
            for _ in 0..sub {
                mask.push(2);
            }
            Some(Self{
                mask,
            })
        } else {
            None
        }
    }
}

impl Iterator for AddSubCombinations {
    type Item = (Vec<usize>, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        /*
            диффузия во все стороны и пиздец
        */
    }
}

pub fn calculate_size_after_wrap(msg_size: usize) -> usize {
    (if msg_size % 2 == 0 {
        msg_size + 4
    } else {
        msg_size + 5
    } / 2) * 3
}

pub fn wrap_raw(msg: &[u8]) -> (Vec<u8>, usize) {
    let mut msg = checksumm::wrap(msg);
    if msg.len() % 2 != 0 {
        msg.push(0);
    }
    let ecc_len = msg.len() / 2;
    let encoder = reed_solomon::Encoder::new(ecc_len);
    let encoded = encoder.encode(&msg);
    (encoded.to_vec(), ecc_len)
}

pub fn wrap(msg: &[u8]) -> Vec<u8> {
    wrap_raw(msg).0
}

pub fn unwrap_raw(wrapped: &[u8], ecc_len: usize) -> Result<Vec<u8>, ()> {
    let decoder = reed_solomon::Decoder::new(ecc_len);
    match decoder.correct(&wrapped, None) {
        Ok(corrected) => {
            match checksumm::unwrap(corrected.data()) {
                Ok(unwraped) => Ok(unwraped),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

pub fn get_optimal_steps(size: usize) -> usize {
    usize::MAX
}

// TODO Add custom error type
pub fn unwrap(wrapped: &[u8], steps: Option<usize>) -> Result<Vec<u8>, ()> {
    let mut steps = match steps {
        Some(steps) => steps,
        None => get_optimal_steps(wrapped.len()),
    };
    if wrapped.len() % 3 == 0 {
        return unwrap_raw(wrapped, wrapped.len() / 3);
    }
    Err(())
}

#[cfg(test)]
mod ecc_tests {
    use super::*;

    #[test]
    fn add_sub_cmb() {
        let mut iter = AddSubCombinations::new();
        let exp: Vec<Option<(usize, usize)>> = vec![
            Some((0, 0)),
            Some((1, 0)),
            Some((0, 1)),
            Some((1, 1)),
            Some((2, 0)),
            Some((0, 2)),
            Some((2, 1)),
            Some((1, 2)),
            Some((2, 2)),
            Some((3, 0)),
            Some((0, 3)),
            Some((3, 1)),
            Some((1, 3)),
            Some((3, 2)),
            Some((2, 3)),
            Some((3, 3)),
        ];
        let mut real: Vec<Option<(usize, usize)>> = Vec::new();
        for i in 0..exp.len() {
            real.push(iter.next());
        }
        assert_eq!(exp, real);
    }

    #[test]
    fn sizes() {
        for i in 0..100 {
            let msg: Vec<u8> = vec![0; i];
            let exp = calculate_size_after_wrap(msg.len());
            let real = wrap(&msg).len();
            assert_eq!(exp, real);
        }
    }

    #[test]
    fn wrap_unwrap_raw() {
        let msg: &[u8] = b"Some message";
        let (mut wrapped, ecc_len) = wrap_raw(msg);
        let unwrapped = unwrap_raw(&wrapped, ecc_len);
        assert_eq!(Ok(msg.to_vec()), unwrapped);
        for i in 0..wrapped.len() {
            let tmp = wrapped[i];
            for v in 0..=255 {
                if v == tmp { break }
                wrapped[i] = v;
                let unwrapped = unwrap_raw(&wrapped, ecc_len);
                assert_eq!(Ok(msg.to_vec()), unwrapped);
            }
            wrapped[i] = tmp;
        }
    }
}
