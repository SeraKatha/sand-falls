mod single_color_renderer;
pub use single_color_renderer::SingleColorRenderer;
mod canvas;
use canvas::Canvas;
use simulation::Simulation;

pub trait Renderer {
    fn fit_simulation(&mut self, simulation: &Simulation);
    fn render(&mut self, simulation: &Simulation);
}
