use rand::RngExt;

struct GameContext {
    name: String,
    running: bool,
    elapsed_time: f32,
}

trait Game {
    fn context(&self) -> &GameContext;
    fn context_mut(&mut self) -> &mut GameContext;

    fn update(&mut self, dt: f32);
    fn render(&self);

    fn name(&self) -> &str {
        &self.context().name
    }

    fn is_running(&self) -> bool {
        self.context().running
    }
}

struct TestGame {
    context: GameContext,
}

impl Game for TestGame {
    fn context(&self) -> &GameContext {
        &self.context
    }

    fn context_mut(&mut self) -> &mut GameContext {
        &mut self.context
    }

    fn update(&mut self, dt: f32) {
        self.context.elapsed_time += dt;

        // udpate
    }

    fn render(&self) {
        // render
    }
}

fn run(game: &mut dyn Game) {
    println!("Game: {}", game.name());
    while game.is_running() {
        game.update(1.0 / 60.0);
        game.render();
    }
}

fn main() {
    // let mut rng = rand::rng();
    let mut context = GameContext {
        name: String::from("Test Game"),
        running: true,
        elapsed_time: 0.0,
    };

    let mut game = TestGame { context };

    run(&mut game);
}
