pub struct Drawer {
    pub id: i64,
    pub title: String,
    pub operational_bitmap: i64,
}

impl Drawer {
    pub fn is_archived(&self) -> bool {
        self.operational_bitmap & (1 << 3) != 0
    }
}
