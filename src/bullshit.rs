pub fn _get_all_moves_at_position(&self, pos: u64, is_white: bool) -> Vec<u64> {
    let (pawn, knight, bishop, rook, king, pieces) = if is_white {
        (self.get_white_pawns(), self.get_white_knights(), self.get_white_bishops(), self.get_white_rooks(), self.get_white_kings(), self.white_pieces)
    } else {
        (self.get_black_pawns(), self.get_black_knights(), self.get_black_bishops(), self.get_black_rooks(), self.get_black_kings(), self.black_pieces)
    };

    let pos_mask = 1u64 << pos;
    let empty_squares = !pieces;

    if (pawn & pos_mask) == pos_mask {
        find_set_bits_positions(self.get_pawn_move_mask(pos, is_white) & empty_squares)
    } else if (rook & pos_mask) == pos_mask {
        find_set_bits_positions(self.precomps.get_rook_move_mask(pos, self.get_all_pieces()) & empty_squares)
    } else if (bishop & pos_mask) == pos_mask {
        find_set_bits_positions(self.precomps.get_bishop_move_mask(pos, self.get_all_pieces()) & empty_squares)
    } else if (king & pos_mask) == pos_mask {
        find_set_bits_positions(get_king_move_mask(pos) & empty_squares)
    } else if (knight & pos_mask) == pos_mask {
        find_set_bits_positions(self.precomps.get_knight_move_mask(pos) & empty_squares)
    } else {
        find_set_bits_positions(self._get_queen_move_mask(pos) & empty_squares)
    }
}




if is_white {
    if ((self.get_white_pawns() >> pos) & 1u64) == 1 {
        if (1u64 << pos | RANK_2_MASK) == RANK_2_MASK {
            // check for piece in the way
            if 1u64 << (pos-8) | self.get_all_pieces() == self.get_all_pieces(){
                return 0;
            }
            return ((1u64 << (pos-8))|(1u64 << (pos-16))) & !self.get_all_pieces();
        } else {
            return (1u64 << (pos-8)) & !self.get_all_pieces();
        }
    }
} else if ((self.get_black_pawns() >> pos) & 1u64) == 1 {
    if (1u64 << pos | RANK_7_MASK) == RANK_7_MASK {
        if 1u64 << (pos+8) | self.get_all_pieces() == self.get_all_pieces(){
            return 0;
        }                 
        return ((1u64 << (pos+8))|(1u64 << (pos+16))) & !self.get_all_pieces();
    } else {
        return (1u64 << (pos+8)) & !self.get_all_pieces();
    }     
}      
0




if is_white {
    if (self.get_white_pawns() | (1u64 << from)) == self.get_white_pawns() {
        println!("pawn");
        if self.black_pieces | (1u64 << to) == self.black_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                if ((1u64 << to) | RANK_8_MASK) == RANK_8_MASK {
                    self.queen |= 1u64 << to;
                    self.pawn &= !(1u64 << from);
                } else {
                    self.pawn = (self.pa & !(1u64 << from)) | (1u64 << to);
            
                }
                self.last_capturee = 1;
                self._take_piece_at_spot(to, is_white);
                return true;
            } 
        } else if (1u64 << to) == self.en_passant_square {
            self.pawn = (self.get_white_pawns() & !(1u64 << from)) | (1u64 << to);
            self.pawn &= !(1u64 << (to+8));
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
        if (self._get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
            if ((1u64 << to) | RANK_8_MASK) == RANK_8_MASK {
                self.queen |= 1u64 << to;
                self.pawn &= !(1u64 << from);
            } else {
                self.pawn = (self.get_white_pawns() & !(1u64 << from)) | (1u64 << to);
            }
            if from - to == 16 {
                // Set en_passant_square for the next move
                self.en_passant_square = 1u64 << (to + 8);
            } else {
                // uncheck en passant if next move is pawn
                self.en_passant_square = 0;
            }
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        } 
        self.en_passant_square = 0;
    } else if (self.get_white_knights() | (1u64 << from)) == self.get_white_knights() {
        if self.black_pieces | (1u64 << to) == self.black_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.knight = (self.get_white_knights() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 2;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self.precomps.get_knight_move_mask(from) >> to) & 1u64 == 1 {
                self.knight = (self.get_white_knights() & !(1u64 << from)) | (1u64 << to);
                self.last_captured = 0;
                self.last_capturee = 0;    
                return true;
           } 
    } else if (self.get_white_rooks() | (1u64 << from)) == self.get_white_rooks() {
        if self.black_pieces | (1u64 << to) == self.black_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.rook = (self.get_white_rooks() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 3;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self.precomps.get_rook_move_mask(from, self.get_all_pieces()) >> to) & 1u64 == 1 {
                self.rook = (self.get_white_rooks() & !(1u64 << from)) | (1u64 << to);
                self.last_captured = 0;
                self.last_capturee = 0;    
                return true;
            } 
    } else if (self.get_white_bishops() | (1u64 << from)) == self.get_white_bishops() {
        if self.black_pieces | (1u64 << to) == self.black_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.bishop = (self.get_white_bishops() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 4;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self.precomps.get_bishop_move_mask(from, self.get_all_pieces()) >> to) & 1u64 == 1 {
            self.bishop = (self.get_white_bishops() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
            } 
    } else if (self.get_white_queens() | (1u64 << from)) == self.get_white_queens() {
        if self.black_pieces | (1u64 << to) == self.black_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.queen = (self.get_white_queens() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 5;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self._get_queen_move_mask(from) >> to) & 1u64 == 1 {
            self.queen = (self.get_white_queens() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    } else if (self.get_white_kings() | (1u64 << from)) == self.get_white_kings() {
        let old_king = self.get_white_kings();
        self.king = (self.get_white_kings() & !(1u64 << from)) | (1u64 << to);
        let threats = self._get_threat_masks(!is_white).iter().cloned().fold(0, |acc, x| acc | x);
        if (threats & self.get_white_kings()) == self.get_white_kings() {
            self.king = old_king;
            return false;
        }
        self.king = old_king;
        
        if self.white_pieces | (1u64 << to) == self.white_pieces {
            if  (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.king = (self.get_white_kings() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 6;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if ((get_king_move_mask(from)) >> to) & 1u64 == 1 {
            self.king = (self.get_white_kings() & !(1u64 << from)) | (1u64 << to) ;
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    }
} else if (self.get_black_pawns() | (1u64 << from)) == self.get_black_pawns() {
    if self.white_pieces | (1u64 << to) == self.white_pieces {
        if ((1u64 << to) | RANK_1_MASK) == RANK_1_MASK {
            self.queen|= 1u64 << to;
            self.pawn&= !(1u64 << from);
        } else {
            self.pawn= (self.get_black_pawns() & !(1u64 << from)) | (1u64 << to);
        }
        if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
            self.pawn = (self.get_black_pawns() & !(1u64 << from)) | (1u64 << to);
            self.last_capturee = 1;
            self._take_piece_at_spot(to, is_white);
            return true;
        }
    } else if (1u64 << to) == self.en_passant_square {
        self.pawn = (self.get_black_pawns() & !(1u64 << from)) | (1u64 << to);
        self.pawn &= !(1u64 << (to-8));
        self.last_captured = 0;
        self.last_capturee = 0;
        return true;
    } else if (self._get_pawn_move_mask(from, is_white) >> to) & 1u64 == 1 {
        if ((1u64 << to) | RANK_1_MASK) == RANK_1_MASK {
            self.queen|= 1u64 << to;
            self.pawn &= !(1u64 << from);
        } else {
            self.pawn = (self.get_black_pawns() & !(1u64 << from)) | (1u64 << to);
        }

        if to - from == 16 {
            // Set en_passant_square for the next move
            self.en_passant_square = 1u64 << (to - 8);
        } else {
            // uncheck en passant if next move is pawn
            self.en_passant_square = 0;
        }
        return true;
    } 
self.en_passant_square = 0;
} else if (self.get_black_knights() | (1u64 << from)) == self.get_black_knights() {
    if self.white_pieces | (1u64 << to) == self.white_pieces {
        if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
            self.knight = (self.get_black_knights() & !(1u64 << from)) | (1u64 << to);
            self.last_capturee = 2;
            self._take_piece_at_spot(to, is_white);
            return true;
        }  
    } else if (self.precomps.get_knight_move_mask(from) >> to) & 1u64 == 1 {
            self.knight = (self.get_black_knights() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
       } 
    } else if (self.get_black_rooks() | (1u64 << from)) == self.get_black_rooks() {
        if self.white_pieces | (1u64 << to) == self.white_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.rook = (self.get_black_rooks() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 3;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self.precomps.get_rook_move_mask(from, self.get_all_pieces()) >> to) & 1u64 == 1 {
            self.rook = (self.get_black_rooks() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    } else if (self.get_black_bishops() | (1u64 << from)) == self.get_black_bishops() {
        if self.white_pieces | (1u64 << to) == self.white_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.bishop = (self.get_black_bishops() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 4;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self.precomps.get_bishop_move_mask(from, self.get_all_pieces()) >> to) & 1u64 == 1 {
            self.bishop = (self.get_black_bishops() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    } else if (self.get_black_queens() | (1u64 << from)) == self.get_black_queens() {
        if self.white_pieces | (1u64 << to) == self.white_pieces {
            if (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.queen = (self.get_black_queens() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 5;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if (self._get_queen_move_mask(from) >> to) & 1u64 == 1 {
            self.queen = (self.get_black_queens() & !(1u64 << from)) | (1u64 << to);
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    } else if (self.get_black_kings() | (1u64 << from)) == self.get_black_kings() {
        let old_king = self.get_black_kings();
        self.king = (self.get_black_kings() & !(1u64 << from)) | (1u64 << to);
        let threats = self._get_threat_masks(!is_white).iter().cloned().fold(0, |acc, x| acc | x);
        if (threats & self.get_black_kings()) == self.get_black_kings() {
            self.king = old_king;
            return false;
        }
        self.king = old_king;
        
        if self.white_pieces | (1u64 << to) == self.white_pieces {
            if  (self._get_attack_mask(from, is_white) >> to) & 1u64 == 1 {
                self.king = (self.get_black_kings() & !(1u64 << from)) | (1u64 << to);
                self.last_capturee = 6;
                self._take_piece_at_spot(to, is_white);
                return true;
            }  
        } else if ((get_king_move_mask(from)) >> to) & 1u64 == 1 {
            self.king = (self.get_black_kings() & !(1u64 << from)) | (1u64 << to) ;
            self.last_captured = 0;
            self.last_capturee = 0;
            return true;
        }
    }
false
