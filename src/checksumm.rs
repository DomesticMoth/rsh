use blake3;

pub fn wrap(msg: &[u8]) -> Vec<u8> {
    let hash = blake3::hash(msg);
    let hash = hash.as_bytes();
    let prefix: &[u8; 2] = hash[0..2].try_into().unwrap();
    let postfix: &[u8; 2] = hash[2..4].try_into().unwrap();
    let mut ret = Vec::with_capacity(msg.len()+4);
    ret.push(prefix[0]);
    ret.push(prefix[1]);
    for byte in msg {
        ret.push(*byte);
    }
    ret.push(postfix[0]);
    ret.push(postfix[1]);
    ret
}

pub fn unwrap_raw(wrapped: &[u8]) -> Result<Vec<u8>, ()> {
    if wrapped.len() < 5 {
        return Err(())
    }
    let msg_len = wrapped.len() - 4;
    let msg = &wrapped[2..2+msg_len];
    let hash = blake3::hash(msg);
    let hash = hash.as_bytes();
    if hash[0..2] == wrapped[0..2] && hash[2..4] == wrapped[wrapped.len()-2..] {
        Ok(msg.to_vec())
    }else{
        Err(())
    }
}

pub fn unwrap(wrapped: &[u8]) -> Result<Vec<u8>, ()> {
    for from in 0..wrapped.len() {
        for to in (0..wrapped.len()).rev() {
            if to - from < 5 { break };
            if let Ok(unwrapped) = unwrap_raw(&wrapped[from..to+1]){
                return Ok(unwrapped)
            }
        }
        if wrapped[from] != 0 { break }
    }
    Err(())
}

#[cfg(test)]
mod cs_tests {
    use super::*;

    fn add_zeros(msg: &[u8], lead: usize, end: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(msg.len()+4);
        for _ in 0..lead {
            ret.push(0);
        }
        for v in msg {
            ret.push(*v);
        }
        for _ in 0..end {
            ret.push(0);
        }
        ret
    }

    #[test]
      fn wrap_unwrap_raw() {
        let msg: &[u8] = b"Some message";
        let mut wrapped = wrap(msg);
        let unwrapped = unwrap_raw(&wrapped);
        assert_eq!(Ok(msg.to_vec()), unwrapped);
        for i in 0..wrapped.len() {
            let tmp = wrapped[i];
            for v in 0..=255 {
                if v == tmp { break }
                wrapped[i] = v;
                let unwrapped = unwrap_raw(&wrapped);
                assert_eq!(Err(()), unwrapped);
            }
            wrapped[i] = tmp;
        }
    }

    #[test]
      fn wrap_unwrap() {
        let msg: &[u8] = b"Some message";
        let extra_zeros = 10;
        let wrapped_raw = wrap(msg);
        for lead in 0..extra_zeros {
            for end in 0..extra_zeros {
                let mut wrapped = add_zeros(&wrapped_raw, lead, end);
                let unwrapped = unwrap(&wrapped);
                assert_eq!(Ok(msg.to_vec()), unwrapped);
                for i in 0..wrapped.len() {
                    let tmp = wrapped[i];
                    for v in 0..=255 {
                        if v == tmp { break }
                        wrapped[i] = v;
                        let unwrapped = unwrap(&wrapped);
                        assert_eq!(Err(()), unwrapped);
                    }
                    wrapped[i] = tmp;
                }
            }
        }
    }
}
