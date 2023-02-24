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

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    key: u64,
    node_kind: NodeType, // Define in search.rs
    depth: usize,
    score: f64,
}

pub const TT_SIZE: usize = 10usize.pow(8);
static mut TABLEBASE: [Entry; TT_SIZE] = [Entry { key: 0, data: 0.0 }; TT_SIZE];

fn index(key: u64) -> usize {
    let i = key % TT_SIZE as u64;
    i as usize
}

fn mut_entry(key: u64) -> &'static mut Entry {
    unsafe { &mut TABLEBASE[index(key)] }
}
fn const_entry(key: u64) -> &'static Entry {
    unsafe { &TABLEBASE[index(key)] }
}

fn f64_to_u64(val: f64) -> u64 {
    unsafe { std::mem::transmute(val) }
}

pub fn get_entry(key: u64) -> Option<f64> {
    let e = const_entry(key);
    if e.key ^ f64_to_u64(e.data) == key {
        Some(e.data)
    } else {
        None
    }
}
pub fn set_entry(key: u64, data: f64) {
    let mut e = mut_entry(key);
    e.key = key ^ f64_to_u64(data);
    e.data = data;
}
