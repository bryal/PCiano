#![feature(globs)]
extern crate current;
extern crate shader_version;
extern crate input;
extern crate event;
extern crate sdl2;
extern crate sdl2_window;

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rt;
use current::{ Set };
use sdl2_window::Sdl2Window;
// use glfw_window::GlfwWindow;
use input::{keyboard, Keyboard, Mouse};
use event::{
	Events,
	FocusEvent,
	PressEvent,
	// MouseCursorEvent,
	// MouseRelativeEvent,
	// MouseScrollEvent,
	ReleaseEvent,
	RenderEvent,
	ResizeEvent,
	// TextEvent,
	UpdateEvent,
	WindowSettings,
};
use event::window::{CaptureCursor, Ups, MaxFps};
use sound_pa as sound;

mod sound_pa;

// We need to run on the main thread, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
	rt::start(argc, argv, main)
}

fn main() {
	println!("{}", sound::version_text());

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

	let window = RefCell::new(window);
	for e in Events::new(&window).set(Ups(120)).set(MaxFps(60)) {
		e.press(|button| {
			match button {
				Keyboard(key) => {
					if key == keyboard::F12 {
						println!("Switched capture cursor state");
						capture_cursor = !capture_cursor;
						window.borrow_mut().deref_mut()
							.set_mut(CaptureCursor(capture_cursor));
					} else if key == keyboard::Space {
						spawn(proc() sound::play_sin());
					}
				},
				Mouse(button) => (),
			}
		});
		e.release(|button| {
			match button {
				Keyboard(key) => (),
				Mouse(button) => (),
			}
		});
		// e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
		// e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
		// e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
		// e.text(|text| println!("Typed '{}'", text));
		e.resize(|w, h| println!("Resized '{}, {}'", w, h));
		e.focus(|focused| {
			if focused { println!("Gained focus"); }
			else { println!("Lost focus"); }
		});
		e.render(|_| {});
		e.update(|uarg| {
		});
	}
}