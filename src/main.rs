use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

const FPS: f64 = 120f64;

fn move_rect(rect: &mut Rect, dx: i32, dy: i32) {
    rect.set_x(rect.x() + dx);
    rect.set_y(rect.y() + dy);
}

fn reset_bg_color(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(50, 50, 50));
}

fn fps(time_start: std::time::SystemTime) {
    // Calculate the current duration since time_start
    let elapsed_duration = match std::time::SystemTime::now().duration_since(time_start) {
        Ok(dur) => dur,
        Err(_) => {
            return;
        },
    };
    // Calculate the target duration for each frame
    let target_duration = std::time::Duration::from_secs_f64(1.0 / FPS);
    // Check if the elapsed time is less than the target duration
    if elapsed_duration < target_duration {
        // Calculate the remaining time to sleep
        let sleep_duration = target_duration - elapsed_duration;
        // Sleep for the remaining time
        std::thread::sleep(sleep_duration);
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("hardsync", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut rect = Rect::new(100, 100, 100, 100);

    let mut time_start = std::time::SystemTime::now();

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

        if rect.x() + (rect.width() as i32) < 800 {
            move_rect(&mut rect, 1, 0);
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        println!("before drawing rect at {} {}", rect.x(), rect.y());
        _ = canvas.fill_rect(rect);
        println!("after drawing rect at {} {}", rect.x(), rect.y());

        reset_bg_color(&mut canvas);
        canvas.present();
        canvas.clear();

        fps(time_start);
        time_start = std::time::SystemTime::now();
    }

    Ok(())
}
