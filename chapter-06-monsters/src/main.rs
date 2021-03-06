mod component;
mod player;
mod map;
mod rect;
mod visibility_system;
mod monster_ai_system;

use rltk::{GameState, Rltk, RGB, RandomNumberGenerator};
use specs::prelude::*;
use component::*;
use map::*;
use visibility_system::*;
use crate::monster_ai_system::MonsterAI;

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player::player_input(self, ctx);
        self.run_systems();
        draw_map(&self.ecs,ctx);

        //读取 存储容器 中的组件。
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        //关联 坐标位置 和图形渲染
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let  mut gs = State{ecs: World::new()};

    //注册我们创建的组件
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x,y) = room.center();
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1,2);
        match roll {
            1 => {glyph = rltk::to_cp437('g')}
            _ => {glyph = rltk::to_cp437('o')}
        }
        gs.ecs
            .create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster{})
            .build();
    }

    gs.ecs.insert(map);

    //创建实体
    gs.ecs
        .create_entity()
        .with(Position{x: player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();

    rltk::main_loop(context, gs)
}
