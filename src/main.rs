mod framebuffer;
mod vertex;
mod uniforms;
mod noise;
mod shaders;
mod triangle;
mod sphere;
mod renderer;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use uniforms::*;
use sphere::{create_sphere, create_sphere_indices};
use renderer::render;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    // Inicializar ventana con Raylib
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Estrella Animada - Gráficas por Computadora")
        .build();

    rl.set_target_fps(60);

    // Crear framebuffer personalizado
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    framebuffer.set_background_color(Color::new(10, 5, 20, 255));

    // Crear geometría de la esfera (nuestra estrella)
    let slices = 40;
    let stacks = 40;
    let sphere_vertices = create_sphere(1.0, slices, stacks);
    let sphere_indices = create_sphere_indices(slices, stacks);

    // Configurar uniforms (parámetros globales para los shaders)
    let mut uniforms = Uniforms::new();
    
    // Configurar matrices de transformación
    uniforms.projection_matrix = create_perspective_matrix(
        45.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0
    );
    
    uniforms.view_matrix = create_view_matrix(
        Vector3::new(0.0, 0.0, 5.0),  // Posición de la cámara
        Vector3::new(0.0, 0.0, 0.0),   // Hacia dónde mira
        Vector3::new(0.0, 1.0, 0.0)    // Vector "arriba"
    );
    
    uniforms.viewport_matrix = create_viewport_matrix(WIDTH as f32, HEIGHT as f32);
    
    // Parámetros ajustables de la estrella
    uniforms.noise_scale = 2.0;
    uniforms.turbulence_intensity = 0.8;

    // Variables para animación
    let mut time: f32 = 0.0;
    let mut rotation_angle: f32 = 0.0;

    // Loop principal
    while !rl.window_should_close() {
        // Actualizar tiempo
        time += rl.get_frame_time();
        rotation_angle += 0.3 * rl.get_frame_time();
        
        // Actualizar uniforms
        uniforms.time = time;
        
        // Crear matriz de modelo (rotación lenta)
        let rotation_y = create_rotation_y_matrix(rotation_angle);
        let rotation_x = create_rotation_x_matrix(rotation_angle * 0.5);
        let scale = create_scale_matrix(1.5);
        
        uniforms.model_matrix = multiply_matrices(
            &multiply_matrices(&rotation_y, &rotation_x),
            &scale
        );

        // Limpiar framebuffer
        framebuffer.clear();

        // Crear array de vértices usando los índices
        let mut vertex_array = Vec::new();
        for &index in &sphere_indices {
            vertex_array.push(sphere_vertices[index].clone());
        }

        // RENDERIZAR: Aquí es donde ocurre toda la magia
        // 1. Vertex Shader transforma los vértices
        // 2. Se ensamblan triángulos
        // 3. Se rasterizan (convierten a píxeles)
        // 4. Fragment Shader calcula colores finales
        render(&mut framebuffer, &uniforms, &vertex_array);

        // Actualizar textura del framebuffer
        framebuffer.swap_buffers(&mut rl, &thread);

        // Dibujar en pantalla
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // Dibujar el framebuffer
        framebuffer.draw_to_screen(&mut d);
        
        // Mostrar información
        d.draw_text(
            &format!("FPS: {}", d.get_fps()),
            10,
            10,
            20,
            Color::WHITE
        );
        d.draw_text(
            "Estrella Animada con Shaders",
            10,
            HEIGHT as i32 - 30,
            20,
            Color::WHITE
        );
    }
}
