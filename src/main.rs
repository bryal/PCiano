// Copyright (C) 2014  Johan Johansson

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>

extern crate time;
extern crate current;
extern crate shader_version;
extern crate input;
extern crate event;
extern crate sdl2;
extern crate sdl2_window;

use std::cell::RefCell;
use std::rt;
use sdl2_window::Sdl2Window;
// use glfw_window::GlfwWindow;
use current::Set;
use input::{
	keyboard,
	Keyboard,
	Mouse};
use event::{
	Events,
	// FocusEvent,
	PressEvent,
	// MouseCursorEvent,
	// MouseRelativeEvent,
	// MouseScrollEvent,
	ReleaseEvent,
	RenderEvent,
	// ResizeEvent,
	// TextEvent,
	UpdateEvent,
	WindowSettings,
};
use event::window::{
	CaptureCursor,
	Ups,
	MaxFps};
use audio_pa as audio;
use music::Tone;

mod audio_pa;
mod music;

fn max<T: PartialOrd>(a: T, b: T) -> T {
	if a > b { a }
	else { b }
}

// We need to run on the main thread, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
	rt::start(argc, argv, main)
}

fn main() {
	let tones = Vec::from_fn(36, |n| {
		Tone::new(music::note_freq_from_0(n as u16))
	});
	println!("{}", audio::version_text());

	let (t_sndstream, r_sndstream) = sync_channel(0);
	audio::spawn_stream(r_sndstream);

	let (win_width, win_height) = (640, 480);
	let mut window = Sdl2Window::new(
		shader_version::opengl::OpenGL_3_0,
		WindowSettings {
			title: "Keycode".to_string(),
			size: [win_width, win_height],
			fullscreen: false,
			exit_on_esc: true,
			samples: 4,
		}
	);

	println!("Press F12 to turn capture cursor on/off");
	let mut capture_cursor = true;
	window.set_mut(CaptureCursor(capture_cursor));

	let mut a_tone = Tone{ freq: 392.0, amp: 0.0 };
	let mut s_tone = Tone{ freq: 440.0, amp: 0.0 };
	let mut d_tone = Tone{ freq: 493.88, amp: 0.0 };
	let sample_period = 1.0 / audio::default_sample_rate() as f64;
	let mut played_samples = 0_u64;
	let (max_fps, ups) = (60, 120);
	let dt = 1.0 / ups as f64;
	let samples_per_upd = (dt / sample_period) as uint + 2;

	let window = RefCell::new(window);
	for e in Events::new(&window).set(Ups(ups)).set(MaxFps(max_fps)) {
		e.press(|button| {
			match button {
				Keyboard(key) => {
					match key {
						keyboard::F12 => {
							capture_cursor = !capture_cursor;
							window.borrow_mut().deref_mut()
								.set_mut(CaptureCursor(capture_cursor));
						}, keyboard::A => {
							a_tone.amp = 1.0;
						}, keyboard::D => {
							d_tone.amp = 1.0;
						}, keyboard::S => {
							s_tone.amp = 1.0;
						}, _ => ()
					}
					println!("{}", key > keyboard::F);
				}
				Mouse(_) => (),
			}
		});
		e.release(|button| {
			match button {
				Keyboard(_) => (),
				Mouse(_) => (),
			}
		});
		// e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
		// e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
		// e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
		e.render(|_| {});
		e.update(|_| {
			a_tone.change_amp(-2.0 * dt);
			d_tone.change_amp(-2.0 * dt);
			s_tone.change_amp(-2.0 * dt);

			let samples = audio::generate_samples(
				samples_per_upd as uint, sample_period,
				played_samples, &[a_tone,d_tone,s_tone]
			);
			played_samples += samples_per_upd as u64;

			t_sndstream.send(samples);
		});
	}
}