#![feature(globs)]
extern crate native;
extern crate shader_version;
extern crate input;
extern crate event;
extern crate sdl2_window;
// extern crate glfw_window;

use sdl2_window::Sdl2Window;
// use glfw_window::GlfwWindow;
use input::{ keyboard, Keyboard, Mouse };
use event::{
	FocusEvent,
	PressEvent,
	MouseCursorEvent,
	MouseRelativeEvent,
	MouseScrollEvent,
	ReleaseEvent,
	RenderEvent,
	ResizeEvent,
	TextEvent,
	UpdateEvent,
	WindowSettings,
};
use event::window::{ CaptureCursor };

// We need to run on the main thread, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
	native::start(argc, argv, main)
}

fn main() {
	let (WIN_WIDTH, WIN_HEIGHT) = (640, 480);
	let mut window = Sdl2Window::new(
		shader_version::opengl::OpenGL_3_0,
		WindowSettings {
			title: "Keycode".to_string(),
			size: [WIN_WIDTH, WIN_HEIGHT],
			fullscreen: false,
			exit_on_esc: true,
			samples: 4,
		}
	);

	println!("Press C to turn capture cursor on/off");
	window.set_mut(CaptureCursor(true));

	let window = RefCell::new(window);
	for e in Events::new(&window) {
		e.press(|button| {
			match button {
				Keyboard(key) => {
					if key == keyboard::C {
						println!("Turned capture cursor on");
						capture_cursor = !capture_cursor;
						event_iter.window.capture_cursor(capture_cursor);
					}
					println!("Pressed keyboard key '{}'", key);
				},
				Mouse(button) => println!("Pressed mouse button '{}'", button),
			}
		});
		e.release(|button| {
			match button {
				Keyboard(key) => println!("Released keyboard key '{}'", key),
				Mouse(button) => println!("Released mouse button '{}'", button),
			}
		});
		e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
		e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
		e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
		e.text(|text| println!("Typed '{}'", text));
		e.resize(|w, h| println!("Resized '{}, {}'", w, h));
		e.focus(|focused| {
			if focused { println!("Gained focus"); }
			else { println!("Lost focus"); }
		});
		e.render(|_| {});
		e.update(|_| {});
	}
}