// Copyright (C) 2014  Johan Johansson

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>

extern crate portaudio;

use self::portaudio::{pa, types};
use std::num::{Float, FloatMath};
use std::f32;

pub fn version_text() -> String {
	format!("Portaudio: {}", pa::get_version_text())
}

/// Initialize portaudio and create a stream from default ssytem config.
/// Returns tuple with the stream and the `DeviceInfo`.
pub fn create_stream() -> (pa::Stream<f32>, types::DeviceInfo) {
	pa::initialize().unwrap();
	let default_output = pa::device::get_default_output();
	let stream_params_out = types::StreamParameters {
		device: default_output,
		channel_count: 1,
		sample_format: types::Float32,
		suggested_latency: pa::device::get_info(default_output)
			.unwrap().default_low_output_latency
	};
	let mut stream = pa::Stream::new(types::Float32);
	stream.open(None, Some(&stream_params_out), 44100.0, 1024, types::ClipOff)
		.unwrap();

	(stream, pa::device::get_info(default_output).unwrap())
}
	
pub fn play_sin() {
	let bufsize = 512;

	let (mut stream, output_info) = create_stream();
	stream.start().unwrap();

	let period = 1.0 / output_info.default_sample_rate as f32;
	let mut phase = 0_f32;
	for i_t in range(0, 70_u16) {
		let mut buf = Vec::with_capacity(bufsize);
		buf.grow_fn(bufsize, |i|{
			phase += f32::consts::PI * 20.0 * i_t as f32 * period;
			phase.sin()
		});
		stream.write(buf, bufsize as u32);
	}
}
