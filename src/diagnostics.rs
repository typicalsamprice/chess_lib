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

use std::fmt::Debug;

#[cfg(feature = "diagnostics")]
static mut BETA_CUTOFF_COUNT: usize = 0;

#[cfg(feature = "diagnostics")]
static mut QUIESCE_DEPTH: usize = 0;

#[cfg(feature = "diagnostics")]
static mut ALPHABETA_LEAF_NODES_COUNT: usize = 0;

#[cfg(feature = "diagnostics")]
#[macro_export]
macro_rules! debug {
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!());
    };
    ($($X:expr),*) => {
        $(
            eprintln!("[{}:{}:{}] = {:?}", file!(), line!(), column!(), $X);
        )*
    };
}

#[cfg(not(feature = "diagnostics"))]
#[macro_export]
macro_rules! debug {
    () => {};
    ($($X:expr),*) => {};
}

// REAL FUNCTIONS

#[cfg(feature = "diagnostics")]
#[inline(always)]
fn debug<T: Debug>(x: T) {
    eprintln!("[{}:{}:{}] = {x:?}", file!(), line!(), column!());
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn add_beta_cutoffs() {
    unsafe {
        BETA_CUTOFF_COUNT += 1;
    }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn add_quiesce_depth() {
    unsafe {
        QUIESCE_DEPTH += 1;
    }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn add_alphabeta_leaf_nodes() {
    unsafe { ALPHABETA_LEAF_NODES_COUNT += 1; }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn get_beta_cutoffs() -> usize {
    unsafe { BETA_CUTOFF_COUNT }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn get_quiesce_depth() -> usize {
    unsafe { BETA_CUTOFF_COUNT }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn get_alphabeta_leaf_nodes() -> usize {
    unsafe { ALPHABETA_LEAF_NODES_COUNT }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn reset_quiesce_depth() {
    unsafe { QUIESCE_DEPTH = 0; }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn reset_beta_cutoffs() {
    unsafe { BETA_CUTOFF_COUNT = 0; }
}

#[cfg(feature = "diagnostics")]
#[inline(always)]
pub fn reset_alphabeta_leaf_nodes() {
    unsafe { ALPHABETA_LEAF_NODES_COUNT = 0; }
}

// FILLER DIAGNOSTICS

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
fn debug() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn add_beta_cutoffs() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn add_quiesce_depth() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn add_alphabeta_leaf_nodes() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn get_beta_cutoffs() -> usize {
    0
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn get_quiesce_depth() -> usize {
    0
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn get_alphabeta_leaf_nodes() -> usize {
    0
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn reset_quiesce_depth() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn reset_beta_cutoffs() {}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn reset_alphabeta_leaf_nodes() {}
