// mod arithmetic_operation;
mod danmaku;

use danmaku::{Bounds, Spawner, SpiralPattern, Vec2, World};
use macroquad::prelude::*;

#[macroquad::main("Danmaku")]
async fn main() {
    let mut world = World::new();
    let mut spawner = Spawner::new(
        Box::new(SpiralPattern::new(0.0, 400.0, 150.0)),
        0.02,
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
    );

    let mut spawner2 = Spawner::new(
        Box::new(SpiralPattern::new(180.0, 400.0, 150.0)),
        0.02,
        Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
    );

    loop {
        let dt = get_frame_time().min(0.05);

        world.tick(dt);
        world.cull(Bounds::new(
            Vec2::new(10.0, 10.0),
            Vec2::new(screen_width() - 10.0, screen_height() - 10.0),
        ));

        spawner.tick(dt, &mut world);
        spawner2.tick(dt, &mut world);

        clear_background(BLACK);

        for bullet in world.bullets() {
            let position = bullet.position();
            draw_circle(position.x, position.y, 5.0, RED);
        }

        next_frame().await;
    }
}
