#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
#![feature(specialization)]
#![feature(trait_upcasting)]

use test_engine::{paths::home, Screen};

#[macro_use]
extern crate log;

#[allow(unused_imports)]
use crate::benchmark::BenchmarkView;
#[allow(unused_imports)]
use crate::test_game::TestGameView;
#[allow(unused_imports)]
use crate::ui_test::UITestView;

mod benchmark;
mod test_game;
mod ui_test;

#[tokio::main]
async fn main() {
    let mut screen = Screen::new((1000, 600), &home().join("test_engine"));

    screen.ui.set_view::<TestGameView>();
    screen.ui.add_debug_view();

    screen.start_main_loop();
}
