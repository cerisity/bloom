pub struct Map {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = vec![false; width * height];

        for x in 0..width {
            data[x] = true;
            data[(height - 1) * width + x] = true;
        }
        for y in 0..height {
            data[y * width] = true;
            data[y * width + width - 1] = true;
        }

        Map { width, height, data }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return true;
        }
        self.data[y * self.width + x]
    }
}