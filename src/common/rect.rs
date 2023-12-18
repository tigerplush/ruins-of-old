pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x,
            y,
            w,
            h,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
}

#[test]
fn test_center() {
    let room = Rect::new(0, 0, 8, 4);
    assert_eq!(room.center(), (4, 2));
    let room = Rect::new(10, 10, 8, 4);
    assert_eq!(room.center(), (14, 12));
    let room = Rect::new(10, 10, 7, 3);
    assert_eq!(room.center(), (13, 11));
}
