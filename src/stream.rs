

pub struct Stream {
data: Vec<u8>,
index: usize
}

impl Stream {
pub fn new(data: Vec<u8>) -> Stream {
Stream{data: data, index: 0}
}

pub fn get_index(&self) -> usize {
self.index
}

pub fn read_u8(&self) -> u8 {
self.index += 1;
*self.data.get(self.index - 1).unwrap()
}

fn read(&self, l: usize) -> [u8; l] {
self.index += l;
*self.data.get(self.index - l..self.index).unwrap()
}

}