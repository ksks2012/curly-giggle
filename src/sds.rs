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
        SDS {
            len: s.len() as u64,
            free: 0,
            buf: s.as_bytes().to_vec(),
        }
    }

    pub fn sdslen(&self) -> u64 {
        self.len
    }

    pub fn sdsempty(&self) -> bool {
        self.len == 0
    }

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

    pub fn sdsclear(&mut self) {
        self.free += self.len;
        self.buf.clear();
    }

    pub fn sdscat(&mut self, other: &str) {
        self.buf.extend(other.as_bytes());
        self.len += other.len() as u64;
    }

    pub fn sdscatsds(&mut self, other: &SDS) {
        self.buf.extend(other.buf.clone());
        self.len += other.len;
    }

    pub fn sdscpy(&mut self, other: &str) {
        let str_len = other.len() as u64;
        // TODO: if free is less than str_len, then resize the buffer
        if self.free < str_len {
            self.buf.reserve(str_len as usize - self.free as usize);
            self.free = str_len;
        }
        self.buf = other.as_bytes().to_vec();
        self.len = str_len;
    }

    // Give emtpy chars to the SDS's buffer
    pub fn sdsgrowzero(&mut self, len: u64) {
        self.buf.reserve(len as usize);
        self.buf.extend(" ".repeat(len as usize).as_bytes());
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

    // TODO: Remove all of characters in cset from buf of SDS
    pub fn sdstrim(&mut self, _cset: &str) {
    }

    pub fn sdscmp(&self, other: &SDS) -> Ordering {
        self.buf.cmp(&other.buf)
    }

}