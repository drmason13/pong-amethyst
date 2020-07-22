use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, Write, WriteStorage, Entities}, // note, Entities is a specs type alias
    ui::UiText,
};

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH, Frozen};
use crate::systems;

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        WriteStorage<'s, Frozen>, // we use this `Storage` to add the Frozen `Component` to the `Entity` (not `balls` is a Storage of `pong::Ball` Components, no Entities here)
        Entities<'s>,
    );

    fn run(&mut self, (
        mut balls,
        mut transforms,
        mut ui_text,
        mut scores,
        score_text,
        mut frozen,
        entities,
    ): Self::SystemData) {
        for (ball, transform, entity) in (&mut balls, &mut transforms, &*entities).join() {
            let ball_x = transform.translation().x;
            let did_hit = if ball_x < ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                scores.score_right = (scores.score_right + 1)
                    .min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x > ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                scores.score_left = (scores.score_left + 1)
                    .min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };
            
            // freeze ball until after short delay
            if did_hit {
                // move to the middle (so we aren't triggering `did_hit` again and freezing the ball forever)
                transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_WIDTH / 2.0, 0.0);
                // freeze the ball for 1 second:
                systems::frozen::freeze(1.0, entity, &mut frozen);
                // reverse velocity so that it moves the other way after unfreezing
                ball.velocity[0] = -ball.velocity[0];
            }
        }
    }
}