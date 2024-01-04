pub enum Resolution {
    _240p,
    _360p,
    _480p,
    _720p,
    _1080p,
    _2K,
    _4K,
    _5K,
}

impl Resolution {
    pub fn height(&self) -> usize {
        match self {
            Resolution::_240p => 240,
            Resolution::_360p => 360,
            Resolution::_480p => 480,
            Resolution::_720p => 720,
            Resolution::_1080p => 1080,
            Resolution::_2K => 1440,
            Resolution::_4K => 2160,
            Resolution::_5K => 2880,
        }
    }
}
