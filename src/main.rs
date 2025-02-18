mod board;
mod precomps;
mod uci_wrapper;
mod engine;
mod precomps_pawn_logic;
mod precomps_knight_logic;
mod precomps_bishop_logic;
mod precomps_rook_logic;
mod config;
mod masks;
mod precomps_rook;
mod precomps_bishop;
mod graphics;
mod test_illegal_moves;
mod book_moves;

use board::Chessboard; 
#[allow(unused_imports)]
use graphics::display_bit_board;
use uci_wrapper::uci_loop;
use std::sync::LazyLock;

static PRECOMPS: LazyLock<precomps::Precomps> = LazyLock::new(|| precomps::Precomps::new());

fn main() {
    let chessboard = Chessboard::new(&PRECOMPS); // Pass a reference to precomps

    uci_loop(chessboard);
}