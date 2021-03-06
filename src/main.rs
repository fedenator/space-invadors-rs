
use sdl2::event::{ Event };

fn main() {
    let sdl = sdl2::init().unwrap();
	let video_subsystem = sdl.video().unwrap();
	let window = video_subsystem
		.window("Game", 900, 700)
		.opengl()
		.resizable()
		.build()
		.unwrap();

	let mut event_pump = sdl.event_pump().unwrap();
	
	'main: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => break 'main,
				_ => {},
			};
		}
		// Aca va lo de pitnar
	}
}
