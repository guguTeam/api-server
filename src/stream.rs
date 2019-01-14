use std::io;

pub struct Stream {
    data: Vec<u8>,
    index: usize,
}

impl Stream {

    #[inline]
    pub fn new(data: Vec<u8>) -> Stream {
        Stream { data: data, index: 0 }
    }

    #[inline]
    pub fn get_index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.read(1).pop().unwrap()
    }

    #[inline]
    pub fn read_str(&mut self, l: usize) -> String {
        String::from_utf8(self.read(l)).unwrap()
    }

    #[inline]
    pub fn read_u8_array(&mut self, l: usize) -> Vec<u8> {
        self.read(l)
    }

    pub fn check_len(&self, l: usize) -> Result<(), io::Error> {
        if self.data.len() != l {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("数据包长度错误。期望{}， 实际{}", l, self.data.len())))
        }else {
            Ok(())
        }
    }

    #[inline]
    fn read(&mut self, l: usize) -> Vec<u8> {
        self.index += l;
        (*self.data.get(self.index - l..self.index).unwrap()).to_vec()
    }
}