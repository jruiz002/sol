use raylib::prelude::*;
use crate::vertex::{Vertex, Fragment};
use crate::uniforms::{Uniforms, multiply_matrix_vector, multiply_matrices};
use crate::noise::{turbulence, cellular_noise};

/// VERTEX SHADER
/// Transforma los vértices del espacio local al espacio de pantalla
/// Aplica las transformaciones de modelo, vista, proyección y viewport
pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Combinar todas las matrices de transformación
    let mvp = multiply_matrices(
        &multiply_matrices(
            &uniforms.projection_matrix,
            &uniforms.view_matrix
        ),
        &uniforms.model_matrix
    );
    
    // Transformar posición del vértice
    let clip_position = multiply_matrix_vector(&mvp, vertex.position);
    
    // Aplicar matriz de viewport para convertir a coordenadas de pantalla
    let screen_position = multiply_matrix_vector(&uniforms.viewport_matrix, clip_position);
    
    // Calcular posición en espacio mundo (sin proyección)
    let world_position = multiply_matrix_vector(&uniforms.model_matrix, vertex.position);
    
    // DISTORSIÓN DEL VERTEX SHADER - simula actividad solar
    let noise_offset = turbulence(
        world_position.x * 2.0,
        world_position.y * 2.0,
        world_position.z * 2.0 + uniforms.time * 0.5,
        3
    ) * 0.1 * uniforms.turbulence_intensity;
    
    // Aplicar distorsión a lo largo de la normal
    let distorted_position = Vector3::new(
        world_position.x + vertex.normal.x * noise_offset,
        world_position.y + vertex.normal.y * noise_offset,
        world_position.z + vertex.normal.z * noise_offset,
    );
    
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        transformed_position: screen_position,
        world_position: distorted_position,
    }
}

/// FRAGMENT SHADER
/// Calcula el color final de cada píxel
/// Implementa la apariencia de la estrella con ruido, colores dinámicos y emisión
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let pos = fragment.world_position;
    let time = uniforms.time;
    
    // 1. RUIDO BASE - Usar turbulencia de Perlin para patrones complejos
    let noise_scale = uniforms.noise_scale;
    let turbulence_value = turbulence(
        pos.x * noise_scale,
        pos.y * noise_scale,
        pos.z * noise_scale + time * 0.3,
        4
    );
    
    // 2. CELLULAR NOISE - Para manchas solares
    let cellular = cellular_noise(pos, 3.0 + (time * 0.2).sin() * 0.5);
    let solar_spots = (cellular * 5.0).sin() * 0.3;
    
    // 3. ANIMACIÓN DE PULSACIÓN - Simula pulsaciones de la estrella
    let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.2 + 0.8;
    
    // 4. CÁLCULO DE INTENSIDAD - Combina todos los efectos
    let base_intensity = turbulence_value * pulse;
    let intensity = (base_intensity - solar_spots).clamp(0.0, 1.0);
    
    // 5. GRADIENTE DE TEMPERATURA - Color basado en intensidad (negro -> rojo -> naranja -> amarillo -> blanco)
    let color = temperature_to_color(intensity);
    
    // 6. EMISIÓN VARIABLE - Picos de energía
    let emission_boost = ((pos.x + pos.y + pos.z) * 10.0 + time * 5.0).sin() * 0.2 + 1.0;
    
    // 7. EFECTO DE FLARE - Brillo adicional en los bordes
    let distance_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    let flare = (1.0 - distance_from_center).max(0.0) * 0.3;
    
    // Color final con todos los efectos
    let final_r = (color.x * emission_boost + flare).clamp(0.0, 1.0);
    let final_g = (color.y * emission_boost + flare * 0.7).clamp(0.0, 1.0);
    let final_b = (color.z * emission_boost + flare * 0.3).clamp(0.0, 1.0);
    
    Color::new(
        (final_r * 255.0) as u8,
        (final_g * 255.0) as u8,
        (final_b * 255.0) as u8,
        255
    )
}

/// Convierte intensidad a color basado en temperatura de estrella
/// Simula el espectro de cuerpo negro
fn temperature_to_color(intensity: f32) -> Vector3 {
    if intensity < 0.3 {
        // Negro a rojo oscuro (manchas solares frías)
        let t = intensity / 0.3;
        Vector3::new(t * 0.5, 0.0, 0.0)
    } else if intensity < 0.5 {
        // Rojo oscuro a rojo brillante
        let t = (intensity - 0.3) / 0.2;
        Vector3::new(0.5 + t * 0.5, t * 0.1, 0.0)
    } else if intensity < 0.7 {
        // Rojo a naranja
        let t = (intensity - 0.5) / 0.2;
        Vector3::new(1.0, 0.1 + t * 0.5, 0.0)
    } else if intensity < 0.85 {
        // Naranja a amarillo
        let t = (intensity - 0.7) / 0.15;
        Vector3::new(1.0, 0.6 + t * 0.4, t * 0.2)
    } else {
        // Amarillo a blanco (zonas muy calientes)
        let t = (intensity - 0.85) / 0.15;
        Vector3::new(1.0, 1.0, 0.2 + t * 0.8)
    }
}
