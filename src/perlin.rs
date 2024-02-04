// vim: set colorcolumn=100:

/// Improved 1D Perlin Noise.
///
/// This is a Rust translation of the Java code for Improved Perlin Noise, found at:
///   https://mrl.cs.nyu.edu/~perlin/noise/
/// Eternal thanks to Ken Perlin as always.
///
/// Returns values in the range `[-1.0, 1.0]`.
///
/// # Arguments
///
/// * `x` - x-coordinate.
/// * `y` - y-coordinate.
/// * `z` - z-coordinate.
pub fn noise(x: f32, y: f32, z: f32) -> f32 {
    let (x_cube, x) = ucu(x);
    let (y_cube, y) = ucu(y);
    let (z_cube, z) = ucu(z);

    let u: f32 = fade(x);
    let v: f32 = fade(y);
    let w: f32 = fade(z);

    let a = p(x_cube) + y_cube;
    let aa = p(a) + z_cube;
    let ab = p(a + 1) + z_cube;
    let b = p(x_cube + 1) + y_cube;
    let ba = p(b) + z_cube;
    let bb = p(b + 1) + z_cube;

    lerp(
        w,
        lerp(
            v,
            lerp(u, grad(p(aa), x, y, z), grad(p(ba), x - 1.0, y, z)),
            lerp(
                u,
                grad(p(ab), x, y - 1.0, z),
                grad(p(bb), x - 1.0, y - 1.0, z),
            ),
        ),
        lerp(
            v,
            lerp(
                u,
                grad(p(aa + 1), x, y, z - 1.0),
                grad(p(ba + 1), x - 1.0, y, z - 1.0),
            ),
            lerp(
                u,
                grad(p(ab + 1), x, y - 1.0, z - 1.0),
                grad(p(bb + 1), x - 1.0, y - 1.0, z - 1.0),
            ),
        ),
    )
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn grad(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 0x0f;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    let u_comp = if h & 1 == 0 { u } else { -u };
    let v_comp = if h & 2 == 0 { v } else { -v };

    u_comp + v_comp
}

/// Find unit-cube coordinate and offset into the unit cube.
fn ucu(x: f32) -> (i32, f32) {
    let x_i32: i32 = (clamp(i32::MIN as f32, i32::MAX as f32, x).floor() as i32) & 255;
    let x = x - x_i32 as f32;
    (x_i32, x)
}

fn clamp(min: f32, max: f32, x: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Look-up an item from the table of permutations.
fn p(i: i32) -> i32 {
    P[(i as usize) % P.len()] as i32
}

/// Table of permutations.
const P: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[cfg(test)]
mod tests {
    use super::*;
    use image;

    /// Compare an x-y slice of Perlin noise at z=0 against a Golden example.
    ///
    /// The Golden example in this case was created by running the original Java reference
    /// implementation of Improved Perlin Noise to create a 32x32 grayscale image, using a noise
    /// cell size of 4 pixels.
    #[test]
    fn perlin_golden_test() {
        // Load the golden test image.
        let golden = load_perlin_4_cell();

        // Create version of the golden test image using our Perlin noise function.
        let result = image::GrayImage::from_fn(32, 32, |x, y| {
            let cell_size = 4.0;
            let x = x as f32 / cell_size;
            let y = y as f32 / cell_size;
            let n = ((0.5 * noise(x, y, 0.0) + 0.5) * 255.0) as u8;
            image::Luma([n])
        });

        // Compare.
        assert_eq!(golden, result)
    }

    /// Load a Perlin golden test image.
    ///
    /// This was created using the original Java Improved Perlin Noise code (in Java).
    /// It contains a slice of the noise at `z=0` with `4x4` cell size.
    fn load_perlin_4_cell() -> image::GrayImage {
        load_perlin_4_cell_r().expect("Could not open Perlin 4-cell noise image.")
    }

    /// Load a Perlin golden test image.
    ///
    /// This was created using the original Java Improved Perlin Noise code (in Java). It contains
    /// a slice of the noise at `z=0` with `4x4` cell size.
    fn load_perlin_4_cell_r() -> image::ImageResult<image::GrayImage> {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let perlin_4_cell_file = format!("{}/test-data/perlin-4-cell.png", manifest);
        let img = image::io::Reader::open(perlin_4_cell_file)?.decode()?;
        let grey_img = img.to_luma8();
        Ok(grey_img)
    }
}
