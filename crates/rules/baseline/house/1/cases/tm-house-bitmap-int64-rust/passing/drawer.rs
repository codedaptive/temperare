pub struct Drawer {
    pub id: i64,
    pub title: String,
    pub operational_bitmap: i64,
}

impl Drawer {
    pub const FLAG_ARCHIVED: i64 = 1 << 3;

    pub fn is_archived(&self) -> bool {
        self.operational_bitmap & Drawer::FLAG_ARCHIVED != 0
    }
}
