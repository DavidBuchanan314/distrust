extern crate sdl2;
use ui::sdl2::pixels::{Color,PixelFormatEnum};
use ui::sdl2::event::Event;
use ui::sdl2::keyboard::Keycode;
use ui::sdl2::rect::Rect;
use std::time::{Instant,Duration};
use std::mem::transmute;

mod vgafont;
use self::vgafont::VGAFONT16;

pub const CGA_PALETTE: &'static [u32] = &[
	0x000000, /* black */
	0x0000aa, /* blue */
	0x00aa00, /* green */
	0x00aaaa, /* cyan */
	0xaa0000, /* red */
	0xaa00aa, /* magenta */
	0xaa5500, /* brown */
	0xaaaaaa, /* grey */
	0x555555, /* dark grey */
	0x5555ff, /* bright blue */
	0x55ff55, /* bright green */
	0x55ffff, /* bright cyan */
	0xff5555, /* bright red */
	0xff55ff, /* bright magenta */
	0xffff55, /* yellow */
	0xffffff, /* white */
];

fn render(texture: &mut sdl2::render::Texture, vram: *const [u8; 0x8000]) {
	texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
		let buffer = unsafe { transmute::<&mut [u8], &mut [u32]>(buffer) };
		let pitch_diff = pitch/4 - 720; // how much "slack space" at the end of each row
		let mut offset = 0;
		for char_y in 0..30 { for subchar_y in 0..16 {
			for char_x in 0..80 {
				let vram_idx = ((char_y*80+char_x)*2) as usize;
				let character = unsafe{ (*vram)[vram_idx] as usize};
				let colour = unsafe{ (*vram)[vram_idx+1] as usize};
				let fg = CGA_PALETTE[colour & 0xF];
				let bg = CGA_PALETTE[(colour & 0xF0) >> 4];
				let mut glyph = VGAFONT16[(character*16+subchar_y) as usize] as u32;
				for subchar_x in 0..9 {
					buffer[offset] = if 0x80 & glyph != 0 { fg } else { bg };
					glyph <<= 1;
					offset += 1;
				}
			}
			offset += pitch_diff;
		}}
	}).unwrap();
}

pub fn ui_main(vram: usize) { // XXX DISGUSTING pointer passing
	let vram = vram as *const [u8; 0x8000];

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("distrust", 720, 480)
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let texture_creator = canvas.texture_creator();
	let mut texture = texture_creator.create_texture_streaming(
		PixelFormatEnum::ARGB8888, 720, 480).unwrap();
	
	
	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running
				},
				_ => {}
			}
		}
		//::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		
		let now = Instant::now();
		render(&mut texture, vram);
		println!("Rendered in {}ms", now.elapsed().subsec_millis());
		
		canvas.copy(&texture, None, Some(Rect::new(0, 0, 720, 480))).unwrap();
		canvas.present();
	}
}

