use raylib::prelude::*;

/// Implementación de Perlin Noise para generar turbulencia en la estrella
/// El ruido de Perlin es una función de ruido de gradiente que produce
/// patrones naturales y suaves

/// Función de interpolación suave (smoothstep)
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Interpolación lineal
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Función de hash simple para generar valores pseudoaleatorios
fn hash(x: i32, y: i32, z: i32) -> f32 {
    let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(z.wrapping_mul(113));
    let n = (n << 13) ^ n;
    let n = (n.wrapping_mul(n.wrapping_mul(n).wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589)) & 0x7fffffff;
    1.0 - (n as f32 / 1073741824.0)
}

/// Producto punto de gradiente
fn grad(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
    
    let u_val = if (h & 1) == 0 { u } else { -u };
    let v_val = if (h & 2) == 0 { v } else { -v };
    
    u_val + v_val
}

/// Perlin Noise 3D - genera ruido suave y coherente
pub fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    // Encontrar la celda unitaria que contiene el punto
    let xi = x.floor() as i32 & 255;
    let yi = y.floor() as i32 & 255;
    let zi = z.floor() as i32 & 255;
    
    // Encontrar posición relativa del punto en la celda
    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();
    
    // Calcular curvas de fade
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);
    
    // Hash de las coordenadas de las 8 esquinas del cubo
    let aaa = hash(xi, yi, zi);
    let aba = hash(xi, yi + 1, zi);
    let aab = hash(xi, yi, zi + 1);
    let abb = hash(xi, yi + 1, zi + 1);
    let baa = hash(xi + 1, yi, zi);
    let bba = hash(xi + 1, yi + 1, zi);
    let bab = hash(xi + 1, yi, zi + 1);
    let bbb = hash(xi + 1, yi + 1, zi + 1);
    
    // Interpolar a lo largo de x
    let x1 = lerp(grad(aaa as i32, xf, yf, zf) as f32, grad(baa as i32, xf - 1.0, yf, zf) as f32, u);
    let x2 = lerp(grad(aba as i32, xf, yf - 1.0, zf) as f32, grad(bba as i32, xf - 1.0, yf - 1.0, zf) as f32, u);
    let y1 = lerp(x1, x2, v);
    
    let x3 = lerp(grad(aab as i32, xf, yf, zf - 1.0) as f32, grad(bab as i32, xf - 1.0, yf, zf - 1.0) as f32, u);
    let x4 = lerp(grad(abb as i32, xf, yf - 1.0, zf - 1.0) as f32, grad(bbb as i32, xf - 1.0, yf - 1.0, zf - 1.0) as f32, u);
    let y2 = lerp(x3, x4, v);
    
    // Interpolar a lo largo de z
    (lerp(y1, y2, w) + 1.0) / 2.0
}

/// Turbulencia - combina múltiples octavas de ruido para crear patrones complejos
pub fn turbulence(x: f32, y: f32, z: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += amplitude * perlin_noise(x * frequency, y * frequency, z * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value
}

/// Cellular/Worley Noise - crea patrones celulares
pub fn cellular_noise(pos: Vector3, scale: f32) -> f32 {
    let p = Vector3::new(pos.x * scale, pos.y * scale, pos.z * scale);
    
    let cell_x = p.x.floor();
    let cell_y = p.y.floor();
    let cell_z = p.z.floor();
    
    let mut min_dist: f32 = 10000.0;
    
    // Buscar en las celdas vecinas
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                let neighbor_x = cell_x + i as f32;
                let neighbor_y = cell_y + j as f32;
                let neighbor_z = cell_z + k as f32;
                
                // Generar punto de característica pseudoaleatorio
                let px = neighbor_x + hash(neighbor_x as i32, neighbor_y as i32, neighbor_z as i32) * 0.5 + 0.25;
                let py = neighbor_y + hash(neighbor_y as i32, neighbor_z as i32, neighbor_x as i32) * 0.5 + 0.25;
                let pz = neighbor_z + hash(neighbor_z as i32, neighbor_x as i32, neighbor_y as i32) * 0.5 + 0.25;
                
                // Calcular distancia al punto de característica
                let dx = p.x - px;
                let dy = p.y - py;
                let dz = p.z - pz;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                
                min_dist = min_dist.min(dist);
            }
        }
    }
    
    min_dist
}

/// Simplex Noise simplificado - alternativa más eficiente a Perlin
pub fn simplex_noise(pos: Vector3, time: f32) -> f32 {
    // Versión simplificada usando Perlin como base
    let noise1 = perlin_noise(pos.x + time * 0.1, pos.y, pos.z);
    let noise2 = perlin_noise(pos.x, pos.y + time * 0.15, pos.z + time * 0.1);
    let noise3 = perlin_noise(pos.x - time * 0.08, pos.y - time * 0.12, pos.z);
    
    (noise1 + noise2 + noise3) / 3.0
}
