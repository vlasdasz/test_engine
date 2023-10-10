use std::path::PathBuf;

use rtools::{init_log, LogBuilder};
use test_engine::{app::AppCore, gm::flat::Size, paths::home, App};
use ui::{
    refs::{enable_ref_stats_counter, Own},
    NavigationView, View,
};

use crate::test_game::TestGameView;

pub struct TestApp {
    core: AppCore,
}

impl App for TestApp {
    fn setup() {
        enable_ref_stats_counter(true);
        init_log(LogBuilder::builder().build());
    }

    fn screen_size() -> Size {
        (1000, 600).into()
    }

    fn make_root_view() -> Own<dyn View> {
        NavigationView::with_view(Own::<TestGameView>::default())
    }

    fn with_core(core: AppCore) -> Self
    where Self: Sized {
        Self { core }
    }

    fn core(&mut self) -> &mut AppCore {
        &mut self.core
    }

    fn assets_path() -> PathBuf {
        home().join("test_engine")
    }
}