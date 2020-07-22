use crate::pong::Frozen;

use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage, Entities},
    ecs,
};

/// Freezes entities (just balls currently, in practice). Frozen entities don't move but retain their velocity
/// This is achieved by having the other movement systems (just move_balls currently, in practice) only act on entities without a Frozen component
/// All this system is responsible for is "unfreezing" Frozen entities when the timer runs down, and ticking that timer (for each entity)
pub struct FreezeSystem;

impl<'s> System<'s> for FreezeSystem {
    type SystemData = (
        WriteStorage<'s, Frozen>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut freezed, time, entities): Self::SystemData) {
        let entities_to_unfreeze = (&mut freezed, &*entities).join()
            .filter_map(|(frozen, entity)| {
                frozen.timer -= time.delta_seconds();
                if frozen.timer <= 0.0 {
                    Some(entity)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
        
        for e in entities_to_unfreeze {
            freezed.remove(e);
        }
    }
}

pub fn freeze(duration: f32, entity: ecs::Entity, frozen: &mut WriteStorage<Frozen>) {
    frozen.insert(entity, Frozen { timer: duration, }).expect("expected to be able to add the frozen component to an entity");
}

