use std::time::Duration;

use sdl2::{
    Sdl, controller::GameController, event::Event, keyboard::Keycode, pixels::Color,
    render::WindowCanvas,
};

const SCREEN_WIDTH: u32 = 224;
const SCREEN_HEIGHT: u32 = 256;

fn video_init(sdl_context: &sdl2::Sdl) -> Result<WindowCanvas, String> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rusty Kong", SCREEN_WIDTH * 3, SCREEN_HEIGHT * 3)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    return Ok(window.into_canvas().build().map_err(|e| e.to_string())?);
}

fn game_controller_init(sdl_context: &Sdl) -> Result<Option<GameController>, String> {
    let game_controller_subsystem = sdl_context.game_controller()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let controller = (0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match game_controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                println!("Success: opened \"{}\"", c.name());
                Some(c)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    });

    let Some(controller) = controller else {
        println!("No usable game controller found. Connect a controller and run again.");
        return Ok(None);
    };

    println!("Controller mapping: {}", controller.mapping());
    return Ok(Some(controller));
}

fn main() -> Result<(), String> {
    // This is required for certain controllers to work on Windows without the
    // video subsystem enabled:
    sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");
    let sdl_context = sdl2::init()?;
    let _controller = game_controller_init(&sdl_context);
    let mut canvas = video_init(&sdl_context)?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
