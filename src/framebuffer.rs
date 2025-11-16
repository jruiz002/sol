use raylib::prelude::*;

/// Framebuffer personalizado para dibujar píxeles manualmente
/// Este es el buffer donde se renderizan todos los píxeles de la estrella
pub struct Framebuffer {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
    pub current_color: Color,
    pub background_color: Color,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    /// Crear un nuevo framebuffer con dimensiones específicas
    pub fn new(width: u32, height: u32) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            pixels: vec![Color::BLACK; total_pixels],
            width,
            height,
            current_color: Color::WHITE,
            background_color: Color::BLACK,
            texture: None,
        }
    }

    /// Limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = self.background_color;
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Establecer un píxel en una posición específica
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    /// Establecer un píxel con un color específico
    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index]
        } else {
            Color::BLACK
        }
    }

    /// Actualizar la textura con los píxeles del framebuffer
    pub fn swap_buffers(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        
        unsafe {
            let image_ptr = image.data as *mut Color;
            for (i, pixel) in self.pixels.iter().enumerate() {
                *image_ptr.add(i) = *pixel;
            }
        }

        if let Some(_old_texture) = self.texture.take() {
            // La textura se libera automáticamente
        }

        match rl.load_texture_from_image(thread, &image) {
            Ok(texture) => self.texture = Some(texture),
            Err(_) => {
                eprintln!("Error cargando textura del framebuffer");
            }
        }
    }

    /// Dibujar el framebuffer en la pantalla
    pub fn draw_to_screen(&self, d: &mut RaylibDrawHandle) {
        if let Some(ref texture) = self.texture {
            d.draw_texture_rec(
                texture,
                Rectangle::new(0.0, 0.0, self.width as f32, -(self.height as f32)),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );
        }
    }

    /// Método auxiliar para dibujar un punto (usado en rasterización)
    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.set_pixel_color(x as u32, y as u32, color);
        }
    }
}
