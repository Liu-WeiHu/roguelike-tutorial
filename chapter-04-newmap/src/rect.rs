
//定义一个矩形
pub(crate) struct Rect {
    pub(crate) x1: i32,
    pub(crate) x2: i32,
    pub(crate) y1: i32,
    pub(crate) y2: i32,
}

impl Rect {
    pub(crate) fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect{x1: x, y1: y, x2: x+w, y2: y+h}
    }

    //如果与other重叠则返回true 四条边只要有相交 就判定为 重叠
    pub(crate) fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub(crate) fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) /2 ,(self.y1 + self.y2) /2)
    }
}