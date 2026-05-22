use simulation::{Cell, Grid, Simulation};
use macroquad::prelude::*;
mod view;
use view::View;

fn init_chunk_texture() -> Texture2D {
    let bytes =  [128u8; Simulation::CELLS_PER_CHUNK * 4];
    Texture2D::from_rgba8(
        Simulation::CHUNK_SIZE as u16,
        Simulation::CHUNK_SIZE as u16,
        &bytes,
    )
}

fn cell_to_color(cell : Cell) -> [u8; 4] {
    match cell {
        Cell::AIR   => [  0,   0,   0, 255],
        Cell::SAND  => [242, 203, 151, 255],
        Cell::STONE => [ 93,  93,  93, 255],
        Cell::WATER => [  0,  96, 255, 255],
    }
}




fn apply_tool(simulation : &mut Simulation, camera : &Camera2D) {
    if macroquad::input::is_mouse_button_down(MouseButton::Left) {
        let mouse_position = macroquad::input::mouse_position(); 
        let global_coord = camera.screen_to_world(vec2(mouse_position.0, mouse_position.1));
        println!("{:?} -> {:?}", mouse_position, global_coord);
        simulation.write_cell(ivec2(global_coord.x as i32, global_coord.y as i32), Cell::SAND);
    }
}


#[macroquad::main("SandFalls")]
async fn main() {
    let world_size = ivec2(128, 128);
    let simulation = Simulation::new(world_size);
    let mut i = 0;
    
    if let Ok(mut simulation) = simulation {
        let num_of_chunks_xy = simulation.num_of_chunks_xy();
        let num_of_chunks_total = simulation.num_of_chunks();
        set_default_filter_mode(FilterMode::Nearest);
        let world_size_f = vec2(world_size.x as f32, world_size.y as f32);
        let mut view = View::new(world_size_f);
        let mut textures : Vec<Texture2D> = std::iter::repeat_with(init_chunk_texture).take(num_of_chunks_total).collect();
        loop {
            let camera = view.into_camera_2d(); 
            set_camera(&camera);
            simulation.tick();
            apply_tool(&mut simulation, &camera);
            simulation.swap_buffers();
            view.update();
            clear_background(BLACK);
            set_camera(&camera);
            for chunk_index in 0..num_of_chunks_total {
                let chunk_coord = Grid::map1Dto2D(chunk_index, num_of_chunks_xy);
                let chunk = simulation.get_chunk(chunk_coord);
                let texture = &mut textures[chunk_index]; 
                
                let mut bytes = [0u8; 4 * Simulation::CELLS_PER_CHUNK];
                for local_index in 0..Simulation::CELLS_PER_CHUNK {
                    let [r, g, b, a] = cell_to_color(chunk[local_index]);
                    bytes[4 * local_index + 0] = r;
                    bytes[4 * local_index + 1] = g;
                    bytes[4 * local_index + 2] = b;
                    bytes[4 * local_index + 3] = a;
                }
                
                texture.update_from_bytes(
                    Simulation::CHUNK_SIZE as u32,
                    Simulation::CHUNK_SIZE as u32,
                    &bytes
                );

                draw_texture(
                    texture,
                    (chunk_coord.x as f32) * (Simulation::CHUNK_SIZE as f32),
                    (chunk_coord.y as f32) * (Simulation::CHUNK_SIZE as f32),
                    WHITE
                );

            }
            // sleep(Duration::from_secs(1));
            
            next_frame().await
        }
    }
}