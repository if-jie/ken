/*
 * @Author: 1454164915@qq.com 1454164915@qq.com
 * @Date: 2024-11-13 14:10:00
 * @LastEditors: 1454164915@qq.com 1454164915@qq.com
 * @LastEditTime: 2024-11-14 13:43:45
 * @FilePath: /ken/src/rect.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */



pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32)->Rect{
        Rect{x1: x, y1: y, x2: x+w, y2: y+h}
    }

    //Return true if this overlaps whith other
    pub fn intersect(&self, other:&Rect)->bool{
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self)->(i32,i32){
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }    
}

