use lab_core::prelude::*;
use systems::npc_move_system;

mod dialog;
mod scripting;
mod systems;

pub mod prelude {
    pub use crate::scripting::*;
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DialogState>()
            .init_resource::<NpcState>()
            .add_event::<NpcEvent>()
            .add_system_to_stage(stage::PRE_UPDATE, npc_move_system.system());
    }
}

// TODO
#[derive(Default)]
struct DialogState {}
#[derive(Default)]
struct NpcState {}
#[derive(Default)]
struct NpcEvent {}
