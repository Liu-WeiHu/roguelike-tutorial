use specs_derive::Component;
use specs::prelude::*;

//定义位置坐标
#[derive(Component)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

//定义渲染组件
#[derive(Component)]
pub(crate) struct Renderable {
    pub(crate) glyph: rltk::FontCharType,
    pub(crate) fg: rltk::RGB,
    pub(crate) bg: rltk::RGB,
}


#[derive(Component, Debug)]
pub(crate) struct Player {}