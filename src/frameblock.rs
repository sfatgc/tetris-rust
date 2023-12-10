pub struct  FrameBlock {
    busy: bool,
    data: char,
}

impl FrameBlock {
    pub fn new() -> FrameBlock { FrameBlock {busy: (false), data: (' ') } }
    pub fn is_busy(&self) -> bool { self.busy }
    pub fn get_data(&self) -> char { self.data }
    pub fn set_data(&mut self, a_data: char) -> char { self.data = a_data; self.get_data() }
}
