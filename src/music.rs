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

use std::num::{
	Float,
	FloatMath};
use std::f64;

pub const A4_STEPS_FROM_0: i16 = 57;
pub const C3_STEPS_FROM_A4: i16 = -21;

#[deriving(Show)]
pub struct Tone {
	pub freq: f64,
	amp: f64,
	pub active: bool
}

impl Tone {
	pub fn new(freq: f64) -> Tone {
		Tone{freq: freq, amp: 0.0, active: false}
	}

	pub fn amp(&self) -> f64 {
		self.amp
	}

	pub fn set_amp(&mut self, val: f64) -> f64 {
		if val < 0.0 {self.amp = 0.0;}
		else if val > 1.0 {self.amp = 1.0;}
		else {self.amp = val;}
		self.amp
	}

	pub fn change_amp(&mut self, val: f64) -> f64 {
		let diff = self.amp + val;
		self.set_amp(diff)
	}
}

/// Returns the frequency of the note placed `n` half steps from A4
// f(n) = f(0) * a^n where f(0) = A4 = 440 and a = 2^(1/12)
pub fn note_freq_from_a4(n: i16) -> f64 {
	let f0 = 440.0;
	let a = 2_f64.powf(1.0/12.0);
	f0 * a.powi(n as i32)
}

pub fn note_freq_from_0(n: u16) -> f64 {
	note_freq_from_a4(n as i16 - A4_STEPS_FROM_0)
}

pub fn generate_samples(bufsize: uint, sample_period: f64, played_samples: u64,
	tones: &[Tone]) -> Vec<f32>
{
	Vec::from_fn(bufsize, |i| {
		(tones.iter().fold(0.0, |sum, t| {
			// Radians per sample
			let rads = 2.0 * f64::consts::PI * t.freq * sample_period;
			let v = rads * (i as u64 + played_samples + 1) as f64;
			v.sin() * t.amp() + sum
		}) / tones.len() as f64 * 4.0) as f32
	})
}