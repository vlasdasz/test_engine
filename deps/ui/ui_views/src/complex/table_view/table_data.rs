use std::{any::Any, fmt::Debug};

use refs::Own;
use ui::View;

pub trait TableData: Debug {
    fn cell_height(&self) -> f32;
    fn number_of_cells(&self) -> usize;
    fn make_cell(&self) -> Own<dyn View>;
    fn setup_cell(&self, cell: &mut dyn Any);
    fn setup_cell_for_reuse(&self, cell: &mut dyn Any, index: usize);
    fn cell_selected(&mut self, _index: usize) {}
}
