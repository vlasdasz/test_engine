use gl_wrapper::GLDrawer;
use test_engine::TestScreen;

fn main() {
    GLDrawer::<TestScreen>::with_size((1200, 600).into()).start_main_loop();
}
