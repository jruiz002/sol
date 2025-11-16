use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::uniforms::Uniforms;
use crate::shaders::{vertex_shader, fragment_shader};
use crate::triangle::triangle;

/// PIPELINE DE RENDERIZADO COMPLETO
/// Este es el proceso que transforma vértices 3D en píxeles en la pantalla
pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // ==========================================
    // ETAPA 1: VERTEX SHADER
    // Transforma cada vértice del espacio local al espacio de pantalla
    // Aplica todas las matrices de transformación (modelo, vista, proyección)
    // ==========================================
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // ==========================================
    // ETAPA 2: PRIMITIVE ASSEMBLY
    // Agrupa los vértices transformados en primitivas (triángulos)
    // Cada 3 vértices consecutivos forman un triángulo
    // ==========================================
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // ==========================================
    // ETAPA 3: RASTERIZACIÓN
    // Convierte cada triángulo en fragmentos (píxeles candidatos)
    // Usa coordenadas baricéntricas para interpolar atributos
    // ==========================================
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // ==========================================
    // ETAPA 4: FRAGMENT SHADER
    // Calcula el color final de cada fragmento
    // Aplica iluminación, texturas, efectos, etc.
    // ==========================================
    for fragment in fragments {
        let color = fragment_shader(&fragment, uniforms);
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color
        );
    }
}
