use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, Frozen};

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadStorage<'s, Frozen>,
    );

    fn run(&mut self, (balls, mut locals, time, frozen): Self::SystemData) {
        // Move every ball that is not frozen according to its velocity, and the time passed.
        for (ball, local, _) in (&balls, &mut locals, !&frozen).join() {
            local.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}