use rltk::{Rltk, RGB, GameState, Point};
use specs::prelude::*;

mod components;
use components::*;

mod map;
use map::*;

mod player;
use player::*;

mod rect;
pub use rect::Rect;

mod visibility_system;
use visibility_system::VisibilitySystem;

mod monster_ai_system;
use monster_ai_system::*;

mod map_indexing_system;
use map_indexing_system::*;

mod damage_system;
use damage_system::*;

mod melee_combat_system;
use melee_combat_system::*;

mod inventory_system;
use inventory_system::*;

mod gui;

mod gamelog;

mod spawner;

pub struct State {
    pub ecs: World,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem;
        vis.run_now(&self.ecs);

        let mut mob = MonsterAi;
        mob.run_now(&self.ecs);

        let mut mapindex = MapIndexingSystem;
        mapindex.run_now(&self.ecs);

        let mut melee_combat_system = MeleeCombatSystem;
        melee_combat_system.run_now(&self.ecs);

        let mut damage_system = DamageSystem;
        damage_system.run_now(&self.ecs);

        let mut pickup = ItemCollectionSystem;
        pickup.run_now(&self.ecs);

        let mut potions = PotionUseSystem;
        potions.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut new_runstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_runstate = *runstate;
        }

        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                new_runstate = player_input(self, ctx);
            },
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::MonsterTurn;
            },
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            },
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => new_runstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDrinkPotion>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDrinkPotion { potion: item_entity }).expect("Unable to insert intent");
                        new_runstate = RunState::AwaitingInput;
                    }
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = new_runstate;
        }
        damage_system::delete_dead(&mut self.ecs);

        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        gui::draw_ui(&self.ecs, ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToDrinkPotion>();

    let map = Map::new();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }
    gs.ecs.insert(player_entity);

    gs.ecs.insert(map);
    
    gs.ecs.insert(Point::new(player_x, player_y));

    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()]
    });

    rltk::main_loop(context, gs)
}
