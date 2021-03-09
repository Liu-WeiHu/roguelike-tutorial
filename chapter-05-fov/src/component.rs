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

//定义玩家
#[derive(Component, Debug)]
pub(crate) struct Player {}

//定义视线距离
#[derive(Component)]
pub(crate) struct Viewshed {
    pub(crate) visible_tiles: Vec<rltk::Point>,
    pub(crate) range: i32,
    pub(crate) dirty: bool,

}