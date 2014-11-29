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

extern crate portaudio;

use self::portaudio::{
	pa,
	types};
use music;
use std::num::FloatMath;
use std::f64;
use std::comm;


pub fn version_text() -> String {
	format!("Portaudio: {}", pa::get_version_text())
}

fn create_stream() -> pa::Stream<f32> {
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

	stream
}

pub fn default_sample_rate() -> f64 {
	pa::device::get_info(pa::device::get_default_output())
		.unwrap().default_sample_rate
}

pub fn spawn_stream(rx: comm::Receiver<Vec<f32>>) {
	spawn(proc() {
		let mut stream = create_stream();
		stream.start().unwrap();
		for buf in rx.iter() {
			let buflen = buf.len();
			stream.write(buf, buflen as u32);
		}
		stream.close().unwrap();
	});
}

pub fn generate_samples(bufsize: uint, sample_period: f64, played_samples: u64,
	tones: &[music::Tone]) -> Vec<f32>
{
	Vec::from_fn(bufsize, |i| {
		(tones.iter().fold(0.0, |sum, t| {
			// Radians per sample
			let rads = 2.0 * f64::consts::PI * t.freq * sample_period;
			let v = rads * (i as u64 + played_samples + 1) as f64;
			v.sin() * t.amp + sum
		}) / tones.len() as f64) as f32
	})
}
