use bitintr::{Pext, Popcnt};
use nanorand::WyRand;
use nanorand::Rng;

use crate::USE_PEXT;
use crate::bitboard::Bitboard;
use crate::square::Square;
use crate::filerank::{File, Rank};

static mut BISHOP_ATTACK_TABLE: [Bitboard; 0x1480] = [Bitboard::ZERO; 0x1480];
static mut ROOK_ATTACK_TABLE: [Bitboard; 0x19000] = [Bitboard::ZERO; 0x19000];

static mut B_MAGICS: [Magic; 64] = [Magic::nulled(); 64];
static mut R_MAGICS: [Magic; 64] = [Magic::nulled(); 64];

#[derive(Debug, Clone, Copy)]
struct Magic {
    mask: u64,
    magic: u64,
    shift: u32,
    ptr: usize
}

impl Magic {
    #[inline]
    const fn nulled() -> Self {
        Self {
            mask: 0,
            magic: 0,
            shift: 0,
            ptr: 0
        }
    }

    fn index(&self, occ: Bitboard) -> u32 {
        if USE_PEXT {
            return occ.inner().pext(self.mask) as u32;
        }

        if usize::BITS == 64 {
            return (((occ & Bitboard::new(self.mask)).inner().wrapping_mul(self.magic)) >> self.shift) as u32;
        }

        let lo = (occ.inner() as u32) & (self.mask as u32);
        let hi = ((occ.inner() >> 32) as u32) & ((self.mask >> 32) as u32);

        return (lo * (self.magic as u32) ^ hi * ((self.magic >> 32) as u32)) >> self.shift;
    }
}

fn rook_attack(square: Square, occupied: Bitboard) -> Bitboard {
    let sqb = Bitboard::from(square);
    let mut attack = Bitboard::ZERO;
    let edges = (Bitboard::from(File::A)
            | Bitboard::from(File::H)).and_not(square.file())
            | (Bitboard::from(Rank::One)
            | Bitboard::from(Rank::Eight)).and_not(square.rank());

    if occupied.zero() {
        return (Bitboard::from(square.file()) ^ square.rank()) &! edges;
    }

    let mask = edges | occupied;

    for shift in [8_i32, -8, 1, -1] {

        let mut copy = sqb;
        loop {
            if shift >= 0 {
                copy <<= shift as u32;
            } else {
                copy >>= -shift as u32;
            }

            attack |= copy;

            if (copy & mask).nonzero() || copy.zero() { break; }
        }
    }

    attack &! edges
}
fn bishop_attack(square: Square, occupied: Bitboard) -> Bitboard {
    let edges = (Bitboard::from(File::A)
            | Bitboard::from(File::H)).and_not(square.file())
            | (Bitboard::from(Rank::One)
            | Bitboard::from(Rank::Eight)).and_not(square.rank());

    let mask = occupied | edges;

    let sqb = Bitboard::from(square);
    let mut attack = Bitboard::ZERO;

    for shift in [7_i32, -7, 9, -9] {
        let mut copy = sqb;
        loop {
            if shift >= 0 {
                copy <<= shift as u32;
            } else {
                copy >>= -shift as u32;
            }

            attack |= copy;

            if (copy & mask).nonzero() || copy.zero() { break; }
        }
    }

    attack &! edges
}

fn init_magics<const IS_ROOK: bool>(attack_table: &'static mut [Bitboard], magic_table: &'static mut [Magic]) {
    let mut rng = WyRand::new();
    let mut sz = 0;
    let mut cnt = 0;
    let mut b = Bitboard::ZERO;
    let mut occ: [Bitboard; 4096] = [Bitboard::ZERO; 4096];
    let mut refer: [Bitboard; 4096] = [Bitboard::ZERO; 4096];
    let mut epoch: [i32; 4096] = [0; 4096];

    let attack = if IS_ROOK { rook_attack } else { bishop_attack };

    Bitboard::MAX.map_by_square(|s| {
        let ptr = if s.inner() == 0 {
            0
        } else {
            magic_table[s.inner() as usize - 1].ptr + sz
        };
        let m = &mut magic_table[s.inner() as usize];
        m.mask = attack(s, Bitboard::ZERO).inner();
        print!("{s:?}:\n{}", Bitboard::new(m.mask));
        let max: u32 = if usize::BITS == 64 { 64 } else { 32 };
        m.shift = max.abs_diff(m.mask.popcnt() as u32);
        m.ptr = ptr;

        b = Bitboard::ZERO;
        sz = 0;

        loop {
            occ[sz] = b;
            refer[sz] = attack(s, b);

            if USE_PEXT {
                attack_table[b.inner().pext(m.mask) as usize] = refer[sz];
            }

            sz += 1;
            b = Bitboard::new(b.inner().wrapping_sub(m.mask) & m.mask);

            if b.zero() { break; }
        }

        if USE_PEXT { return; }

        let mut i = 0;
        loop {
            m.magic = 0;
            gen_sparse(&mut m.magic, m.mask, &mut rng);
            cnt += 1;
            loop {
                let idx = m.index(occ[i]) as usize;
                if epoch[idx] < cnt {
                    epoch[idx] = cnt;
                    attack_table[m.ptr + idx] = refer[i];
                } else if attack_table[m.ptr + idx] != refer[i] {
                    break;
                }

                i += 1;
                if i >= sz { break; }
            }
            if i >= sz { break; }
        }
    });
}

fn gen_sparse(num: &mut u64, mult: u64, rng: &mut WyRand) {
    loop {
        *num = generate_sparse_u64(rng);

        if (num.wrapping_mul(mult) >> 56).popcnt() >= 6 {
            return;
        }
    }
}
fn generate_sparse_u64(rng: &mut WyRand) -> u64 {
    rng.generate::<u64>()
}
pub fn initalize_magics() {
    unsafe {
        init_magics::<true>(&mut ROOK_ATTACK_TABLE, &mut R_MAGICS);
        init_magics::<false>(&mut BISHOP_ATTACK_TABLE, &mut B_MAGICS);
    }
}

pub fn rook_moves(square: Square, occ: Bitboard) -> Bitboard {
    let magic = unsafe { R_MAGICS[square.inner() as usize] };
    let idx = magic.index(occ);

    unsafe { ROOK_ATTACK_TABLE[idx as usize] }
}
pub fn bishop_moves(square: Square, occ: Bitboard) -> Bitboard {
    let magic = unsafe { B_MAGICS[square.inner() as usize] };
    let idx = magic.index(occ);

    unsafe { BISHOP_ATTACK_TABLE[idx as usize] }
}
