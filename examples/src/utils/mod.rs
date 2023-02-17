pub struct SpinLock<T: Copy> {
    data: Option<T>
}

impl<T: Copy> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: Some(data)
        }
    }
    pub fn lock(&mut self) -> T {
        loop {
            match self.data.take() {
                Some(v) => return v,
                None => {}
            }
        }
    }
    pub fn unlock(&mut self, data: T) {
        self.data = Some(data)
    }
    pub fn copy_to(&mut self, target: &mut T) {
        let data = self.lock();
        *target = data.clone();
        self.unlock(data)
    }
}

pub const SERIAL_BUFF_SIZE: usize = 1024;
#[derive(Copy, Clone)]
pub struct SerialBuff {
    pub length: usize,
    pub data: [u8;SERIAL_BUFF_SIZE]
}
impl SerialBuff {
    pub const fn new() -> Self {
        Self {
            length: 0,
            data: [0;SERIAL_BUFF_SIZE]
        }
    }
    pub fn append_data(&mut self, data: u8) {
        if self.length < SERIAL_BUFF_SIZE {
            self.data[self.length] = data;
            self.length += 1;
        }
    }
}