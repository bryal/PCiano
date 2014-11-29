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

use std::num::Float;

pub struct Tone {
	pub freq: f64,
	pub amp: f64
}

impl Tone {
	pub fn new(freq: f64) -> Tone {
		Tone{freq: freq, amp: 0.0}
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
	let a4_steps_from_0 = 49;
	note_freq_from_a4(n as i16 - a4_steps_from_0)
}