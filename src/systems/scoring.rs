use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH};

pub struct ScoringSystem;

impl<'a> System<'a> for ScoringSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
    );
    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                println!("Player 2 scores!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius { 
                println!("Player 1 scores!");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0);
            }


        }
    }
}