pub struct ResizeItem {
    r: u128,
    g: u128,
    b: u128,
    a: u128,
    count: u128,
}

impl ResizeItem {
    pub fn new() -> Self {
        let (r, g, b, a, count) = (0, 0, 0, 0, 0);
        Self { r, g, b, a, count }
    }

    pub fn push_pixel(&mut self, raw: &[u8; 4]) {
        let [r, g, b, a] = raw.map(|x| x as u128);
        self.r += r;
        self.g += g;
        self.b += b;
        self.a += a;
        self.count += 1;
    }

    pub fn average(&self) -> [u8; 4] {
        let ResizeItem { r, g, b, a, count } = self;
        let average = [r / count, g / count, b / count, a / count];
        average.map(|x| x as u8)
    }
}
