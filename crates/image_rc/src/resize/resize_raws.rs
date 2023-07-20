pub struct ResizeRaws {
    r: u128,
    g: u128,
    b: u128,
    a: u128,
    count: u128,
}

impl ResizeRaws {
    pub fn new() -> Self {
        ResizeRaws {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            count: 0,
        }
    }

    pub fn add_item(&mut self, raw: &[u8; 4]) {
        let raw_u128 = raw.map(|x| x as u128);
        self.r += raw_u128[0];
        self.g += raw_u128[1];
        self.b += raw_u128[2];
        self.a += raw_u128[3];
        self.count += 1;
    }

    pub fn get_average(&self) -> [u8; 4] {
        let ResizeRaws { r, g, b, a, count } = self;
        let average = [r / count, g / count, b / count, a / count];
        average.map(|x| x as u8)
    }
}
