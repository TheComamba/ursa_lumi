use std::f64::consts::PI;

use crate::generation_parameters::GenerationParameters;

/// A chunk of the galaxy.
pub(crate) struct GalacticChunk {
    /// The defining corner of the chunk, in units of parsec.
    ///
    /// The chunk is a cube spanning ().
    pub(crate) corner: (f64, f64, f64),
}

impl GalacticChunk {
    /// Separates the galaxy into chunks, up to the maximal generation distance.
    ///
    /// Compare https://github.com/TheComamba/UrsaLumi/blob/main/Documentation/Generation_Algorithm.md#chunking-the-galaxy
    pub(crate) fn generate_chunks(
        generation_parameters: &GenerationParameters,
    ) -> Vec<GalacticChunk> {
        let n = number_of_chunks_along_axis(generation_parameters);
        let chunks_per_octant = number_of_chunks_per_octant(n);
        let chunk_numbers_first_octant = chunk_numbers_in_first_octant(chunks_per_octant, n);
        create_chunks_in_sphere(generation_parameters, chunk_numbers_first_octant)
    }

    fn origin_chunk(generation_parameters: &GenerationParameters) -> GalacticChunk {
        let mut corner = generation_parameters.observer_position_in_pc;
        let chunksize = generation_parameters.chunksize_in_pc;
        corner.0 -= corner.0 % chunksize;
        corner.1 -= corner.1 % chunksize;
        corner.2 -= corner.2 % chunksize;
        GalacticChunk { corner }
    }
}

fn create_chunks_in_sphere(
    generation_parameters: &GenerationParameters,
    chunk_numbers_first_octant: Vec<(usize, usize, usize)>,
) -> Vec<GalacticChunk> {
    let origin_chunk = GalacticChunk::origin_chunk(generation_parameters);
    let (x0, y0, z0) = origin_chunk.corner;

    let mut chunks: Vec<GalacticChunk> =
        Vec::with_capacity(1 + chunk_numbers_first_octant.len() * 8);
    chunks.push(origin_chunk);
    let size = generation_parameters.chunksize_in_pc;
    for (x, y, z) in chunk_numbers_first_octant.iter() {
        for x_sign in [-1, 1].iter() {
            if x == &0 && x_sign == &-1 {
                continue;
            }
            let x = *x as f64;
            let x_sign = *x_sign as f64;
            for y_sign in [-1, 1].iter() {
                if y == &0 && y_sign == &-1 {
                    continue;
                }
                let y = *y as f64;
                let y_sign = *y_sign as f64;
                for z_sign in [-1, 1].iter() {
                    if z == &0 && z_sign == &-1 {
                        continue;
                    }
                    let z = *z as f64;
                    let z_sign = *z_sign as f64;
                    let corner = (
                        x0 + x * x_sign * size,
                        y0 + y * y_sign * size,
                        z0 + z * z_sign * size,
                    );
                    chunks.push(GalacticChunk { corner });
                }
            }
        }
    }
    chunks
}

fn chunk_numbers_in_first_octant(chunks_per_octant: usize, n: usize) -> Vec<(usize, usize, usize)> {
    let mut chunks_first_octant = Vec::with_capacity(chunks_per_octant);
    for x in 0..n {
        for y in 0..n {
            if y * y + x * x > n * n {
                continue;
            }
            for z in 0..n {
                if x * x + y * y + z * z > n * n {
                    continue;
                }
                chunks_first_octant.push((x, y, z));
            }
        }
    }
    chunks_first_octant.sort_by_key(|&(x, y, z)| x * x + y * y + z * z);
    chunks_first_octant
}

fn number_of_chunks_per_octant(n: usize) -> usize {
    let chunks_per_octant = (4. / 3. * PI) * (n.pow(3) as f64) / 8.;
    let chunks_per_octant = chunks_per_octant.ceil() as usize;
    chunks_per_octant
}

fn number_of_chunks_along_axis(generation_parameters: &GenerationParameters) -> usize {
    let N = (generation_parameters.max_distance_in_pc / generation_parameters.chunksize_in_pc)
        .floor() as usize;
    N
}
