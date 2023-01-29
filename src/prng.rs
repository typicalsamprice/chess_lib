/*
    ChessLib, a UCI chess engine
    Copyright (C) 2023 Sam Price

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

// Used in magic generation for random u64s
#[derive(Debug)]
pub struct Prng(u64);

impl Prng {
    #[inline]
    pub const fn new(val: u64) -> Self {
        debug_assert!(val > 0);
        Self(val)
    }

    fn sample(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;

        self.0.wrapping_mul(2685821657736338717)
    }

    #[inline]
    pub fn sparse(&mut self) -> u64 {
        self.sample() & self.sample() & self.sample()
    }
}
