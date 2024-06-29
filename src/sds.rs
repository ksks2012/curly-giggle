use std::cmp::Ordering;

// len -> the length of the string
// free -> the number of free bytes in the buffer
// buf -> the buffer that holds the string
#[derive(Debug)]
pub struct SDS {
    len: u64,
    free: u64,
    buf: Vec<u8>,
}

#[allow(dead_code)]
impl SDS {
    pub fn new() -> Self {
        SDS {
            len: 0,
            free: 0,
            buf: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        self.buf.iter().map(|&c| c as char).collect::<String>()
    }

    // Create a new SDS with the given string
    pub fn sdsnew(s: &str) -> Self {
        let buf_len = s.len() * 2;
        let mut sds = SDS {
            len: s.len() as u64,
            free: buf_len as u64 - s.len() as u64,
            buf: vec![0; buf_len],
        };
        let source_slice = s.as_bytes();
        sds.buf[..source_slice.len()].copy_from_slice(source_slice);
        sds
    }

    pub fn sdslen(&self) -> u64 {
        self.len
    }

    pub fn sdsempty(&self) -> bool {
        self.len == 0
    }

    // Free space
    pub fn sdsfree(&mut self) {
        self.len = 0;
        self.free = 0;
        self.buf.clear();
    }

    pub fn sdsavail(&self) -> u64 {
        self.free
    }

    pub fn sdsbuf(&self) -> Vec<u8> {
        self.buf.clone()
    }

    // Create a copy of the current SDS
    pub fn sdsdup(&self) -> Self {
        SDS {
            len: self.len,
            free: self.free,
            buf: self.buf.clone(),
        }
    }

    // Clear buf (inertia)
    pub fn sdsclear(&mut self) {
        self.len = 0;
        self.free += self.len;
        self.buf[0] = 0;
    }

    // Add string to end of SDS
    pub fn sdscat(&mut self, other: &str) {
        // Check available space
        let str_len = other.len() as u64;
        if self.free < str_len {
            let new_buf_len = (self.len + str_len) * 2;
            let mut new_buf = vec![0; new_buf_len as usize];
            new_buf[..self.len as usize].copy_from_slice(&self.buf[..self.len as usize]);
            new_buf[self.len as usize..(self.len + str_len) as usize].copy_from_slice(other.as_bytes());
            self.buf = new_buf;
            self.free = new_buf_len - self.len - str_len;
        } else {
            let source_slice = other.as_bytes();
            self.buf[..source_slice.len()].copy_from_slice(source_slice);
            self.free -= str_len;
        }
        self.len += str_len;
    }

    pub fn sdscatsds(&mut self, other: &SDS) {
        let other_len = other.sdslen();
        let other_str = other.to_string();
        let (prefix, _suffix) = other_str.split_at(other_len as usize);
        self.sdscat(prefix);
    }

    // Copy input string to buf
    pub fn sdscpy(&mut self, other: &str) {
        let str_len = other.len() as u64;
        // If len free is less than str_len, then resize the buffer
        if self.len + self.free < str_len {
            let new_buf_len = str_len * 2;
            let mut new_buf = vec![0; new_buf_len as usize];
            new_buf[..str_len as usize].copy_from_slice(other.as_bytes());
            self.buf = new_buf;
            self.free = new_buf_len - str_len;
        } else {
            let source_slice = other.as_bytes();
            self.buf[..source_slice.len()].copy_from_slice(source_slice);
            self.free = self.free + self.len - str_len;
        }
        self.buf[other.len()] = 0;
        self.len = str_len;
    }

    // Give emtpy chars to the SDS's buffer
    pub fn sdsgrowzero(&mut self, len: u64) {
        self.buf.reserve(len as usize);
        self.buf.extend("\0".repeat(len as usize).as_bytes());
        self.free += len;
    }
    
    // Restore range of the SDS, clear others
    /*
     * TODO: start, end can be negative, -1 means the last character of the
     * string, -2 the penultimate character, and so forth.
     */
    pub fn sdsrange(&mut self, mut start: usize, mut end: usize) -> i64 {
        // TODO: Check len with SSIZE_MAX
        if self.len == 0 {
            return 0
        }
        let mut new_len = end - start + 1;
        if new_len != 0 {
            if start >= self.len as usize {
                new_len = 0
            } else if end >= self.len as usize {
                end = self.len as usize - 1;
                new_len = end - start + 1
            }
        } else {
            start = 0
        }
        if start != 0 && new_len != 0 {
            // memmove
            self.buf.copy_within(start..(end+1), 0)
        }
        self.buf[new_len as usize] = 0;
        // Update len & free
        self.free = self.free + self.len - new_len as u64;
        self.len = new_len as u64;

        return 0
    }

    // Remove all of characters in cset from buf of SDS
    pub fn sdstrim(&mut self, cset: &str) {
        let mut new_str = self.to_string();
        let mut sp = 0 as usize;
        let mut ep = (self.len - 1) as usize;
        let end = ep;

        let s_bytes = unsafe { new_str.as_mut_vec() }; // Get a mutable reference to the byte array of the string

        while sp <= end && cset.contains(s_bytes[sp] as char) {
            sp += 1;
        }
        while ep > sp && cset.contains(s_bytes[ep] as char) {
            ep -= 1;
        }

        let len = ep - sp + 1;
        if sp > 0 {
            s_bytes.copy_within(sp..=ep, 0); // Shift the content to the start
        }
        s_bytes.truncate(len); // Truncate the vector to the new length
        self.sdscpy(&new_str)
    }

    pub fn sdscmp(&self, other: &SDS) -> Ordering {
        self.buf.cmp(&other.buf)
    }

}