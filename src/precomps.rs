use std::collections::HashMap;
use crate::precomps_knight_logic::*;
use crate::precomps_rook_logic::*;
use crate::config::*;
use crate::rook_magics::*;

#[derive(Clone)]
pub struct Precomps {
    knight_table: HashMap<u64, u64>,
    rook_entries: [MagicEntry; 64],
    rook_table: [u64; 102400],
    rook_table_offsets: [usize; 64]
}

#[derive(Clone)]
#[derive(Copy)]
pub struct MagicEntry {
    pub magic: u64,
    pub mask: u64,
    pub shift: u8,
}


impl Precomps {
    pub fn new() -> Precomps {
        if PRECOMP_ROOK {init_rook_magics(ROOK_FILE_PATH);};
        Precomps {
            knight_table: init_knight_and_masks(),
            rook_entries: ROOK_MAGIC_ENTRIES,
            rook_table: ROOK_MOVE_TABLE,
            rook_table_offsets: ROOK_MOVE_TABLE_OFFSETS,
        }
    }

    pub fn get_knight_move_mask(&self, pos: u64) -> u64 {
        return self.knight_table.get(&pos).unwrap().clone();
    }

    pub fn magic_index(&self, entry: &MagicEntry, blockers: u64) -> u64{
        let blockers_that_matter = blockers & entry.mask;
        let magic_mul = blockers_that_matter.wrapping_mul(entry.magic);
        return magic_mul >> (entry.shift);
    }

    pub fn get_rook_move_mask(&self, pos: u64, blockers: u64) -> u64 {
        let offset = self.rook_table_offsets[pos as usize];
        let index = self.magic_index(&self.rook_entries[pos as usize], blockers) as usize;
    
        self.rook_table[offset + index]
    }

}
