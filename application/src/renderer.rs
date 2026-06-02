mod single_color_renderer;
mod textured_renderer;
mod canvas;

pub use single_color_renderer::SingleColorRenderer;
pub use textured_renderer::TexturedRenderer;
use canvas::Canvas;

use simulation::Simulation;

pub trait Renderer {
    fn fit_simulation(&mut self, simulation: &Simulation);
    fn render(&mut self, simulation: &Simulation);
}
