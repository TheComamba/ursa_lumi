use std::cmp::min;
use std::f64::consts::PI;

use crate::generation_parameters::GenerationParameters;
use crate::MAX_ITEMS_IN_VECTOR;

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
    pub(crate) fn generate_chunks(generation_parameters: &GenerationParameters) -> Vec<Vec<Self>> {
        let n = number_of_chunks_along_axis(generation_parameters);
        let chunk_numbers_first_octant = chunk_numbers_in_first_octant(n);
        Self::create_chunks_in_sphere(generation_parameters, chunk_numbers_first_octant)
    }

    fn origin_chunk(generation_parameters: &GenerationParameters) -> Self {
        let mut corner = generation_parameters.observer_position_in_pc;
        let chunksize = generation_parameters.chunksize_in_pc;
        corner.0 -= corner.0 % chunksize;
        corner.1 -= corner.1 % chunksize;
        corner.2 -= corner.2 % chunksize;
        Self { corner }
    }

    fn create_limited_number_of_chunks_in_sphere(
        generation_parameters: &GenerationParameters,
        chunk_numbers_first_octant: Vec<(usize, usize, usize)>,
    ) -> Vec<Self> {
        let origin_chunk = Self::origin_chunk(generation_parameters);
        let (x0, y0, z0) = origin_chunk.corner;

        let mut chunks: Vec<Self> = Vec::with_capacity(1 + chunk_numbers_first_octant.len() * 8);
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

    #[cfg(test)]
    fn eq(&self, other: &Self) -> bool {
        const ACCURACY: f64 = 1e-2;
        let (x0, y0, z0) = self.corner;
        let (x1, y1, z1) = other.corner;
        (x0 - x1).abs() < ACCURACY && (y0 - y1).abs() < ACCURACY && (z0 - z1).abs() < ACCURACY
    }
}

fn chunk_numbers_in_first_octant(target_radius: usize) -> Vec<Vec<(usize, usize, usize)>> {
    const EMPTY: &'static [(usize, usize, usize)] = &[];
    let total_chunks = approximate_number_of_chunks_in_sphere(target_radius);
    let mut min_radius = 0;
    let mut max_radius = min(target_radius, radius_of_sphere(MAX_ITEMS_IN_VECTOR));
    let mut chunks = Vec::new();
    while min_radius < target_radius {
        let previous_chunks = if let Some(last) = chunks.last() {
            last
        } else {
            &Vec::new()
        };
        let new_chunks = some_chunk_numbers_in_first_octant(min_radius, max_radius, previous_chunks);
        chunks.push(new_chunks);
        let remaining_chunks = total_chunks - total_number(&chunks);
        min_radius = max_radius;
        max_radius = min(target_radius, ));
    }
    chunks
}

fn some_chunk_numbers_in_first_octant(
    r_min: usize,
    r_max: usize,
    inner_shell: &Vec<(usize, usize, usize)>,
) -> Vec<(usize, usize, usize)> {
    let mut chunks_first_octant = Vec::new();
    for x in 0..r_max {
        for y in 0..r_max {
            if y * y + x * x > r_max * r_max {
                continue;
            }
            for z in 0..r_max {
                if x * x + y * y + z * z > r_max * r_max {
                    continue;
                }
                if x * x + y * y + z * z <= r_min * r_min {
                    if inner_shell.contains(&(x, y, z)) {
                        continue;
                    }
                }
                chunks_first_octant.push((x, y, z));
            }
        }
    }
    chunks_first_octant.sort_by_key(|&(x, y, z)| x * x + y * y + z * z);
    chunks_first_octant
}

fn number_of_chunks_along_axis(generation_parameters: &GenerationParameters) -> usize {
    (generation_parameters.max_distance_in_pc / generation_parameters.chunksize_in_pc).ceil()
        as usize
}

fn approximate_number_of_chunks_in_sphere(n: usize) -> usize {
    (4. / 3. * PI * n.pow(3) as f64 ).round() as usize
}

fn radius_of_sphere(chunks_inside: usize) -> usize {
    ((3. / 4. * chunks_inside as f64 / PI).powf(1. / 3.)).ceil() as usize
}

fn total_number<T>(v: &Vec<Vec<T>>) -> usize {
    v.iter().map(|v| v.len()).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_number_of_chunks_along_axis() {
        let params = GenerationParameters {
            observer_position_in_pc: (0., 0., 0.),
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: 5.,
            chunksize_in_pc: 10.,
        };
        assert_eq!(number_of_chunks_along_axis(&params), 1);

        let params = GenerationParameters {
            observer_position_in_pc: (0., 0., 0.),
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: 55.,
            chunksize_in_pc: 10.,
        };
        assert_eq!(number_of_chunks_along_axis(&params), 6);
    }

    #[test]
    fn test_chunk_numbers_in_first_octant() {
        assert_eq!(chunk_numbers_in_first_octant(1), vec![vec![(0, 0, 0)]]);
        assert_eq!(
            chunk_numbers_in_first_octant(2),
            vec![vec![
                (0, 0, 0),
                (0, 0, 1),
                (0, 1, 0),
                (1, 0, 0),
                (0, 1, 1),
                (1, 0, 1),
                (1, 1, 0),
                (1, 1, 1)
            ]]
        );
    }

    #[test]
    fn flattened_chunk_numbers_are_the_same_even_if_generated_in_batches() {
        const R_MAX: usize = 100;
        let all_chunk_numbers = some_chunk_numbers_in_first_octant(0, R_MAX, &vec![]);
        for r_min in 1..(R_MAX - 1) {
            let inner_shell = some_chunk_numbers_in_first_octant(0, r_min, &vec![]);
            let outer_shell = some_chunk_numbers_in_first_octant(r_min, R_MAX, &inner_shell);
            let mut combined = inner_shell.clone();
            combined.extend(outer_shell);
            assert_eq!(all_chunk_numbers, combined);
        }
    }

    #[test]
    fn chunks_with_small_max_generation_distance_contain_origin_chunk() {
        let origin = (10., 20., 30.);
        let params = GenerationParameters {
            observer_position_in_pc: origin,
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: 0.1,
            chunksize_in_pc: 10.,
        };
        let origin_chunk = GalacticChunk::origin_chunk(&params);

        let chunks = GalacticChunk::generate_chunks(&params);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].eq(&origin_chunk));
    }

    #[test]
    fn chunks_do_not_contain_duplications() {
        for n in 1..10 {
            let params = GenerationParameters {
                observer_position_in_pc: (0., 0., 0.),
                apparent_magnitude_limit: 0.,
                max_distance_in_pc: 10. * n as f64,
                chunksize_in_pc: 10.,
            };
            let chunks = GalacticChunk::generate_chunks(&params);
            for (i, chunk) in chunks.iter().enumerate() {
                for other_chunk in chunks.iter().skip(i + 1) {
                    assert!(!chunk.eq(other_chunk));
                }
            }
        }
    }

    #[test]
    fn chunks_are_sorted_by_distance() {
        const ACCURACY: f64 = 1.;
        let params = GenerationParameters {
            observer_position_in_pc: (0., 0., 0.),
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: 101.,
            chunksize_in_pc: 10.,
        };
        let (x0, y0, z0) = GalacticChunk::origin_chunk(&params).corner;
        let chunks = GalacticChunk::generate_chunks(&params);
        for (i, chunk) in chunks.iter().enumerate() {
            for other_chunk in chunks.iter().skip(i + 1) {
                let (x1, y1, z1) = chunk.corner;
                let (dx1, dy1, dz1) = (x1 - x0, y1 - y0, z1 - z0);
                let (x2, y2, z2) = other_chunk.corner;
                let (dx2, dy2, dz2) = (x2 - x0, y2 - y0, z2 - z0);
                let distance1 = dx1 * dx1 + dy1 * dy1 + dz1 * dz1;
                let distance2 = dx2 * dx2 + dy2 * dy2 + dz2 * dz2;
                assert!(distance1 <= distance2 + ACCURACY);
            }
        }
    }

    #[test]
    fn radius_roundtrips() {
        for r1 in 1..100 {
            let chunks_inside = approximate_number_of_chunks_in_sphere(r1);
            let r2 = radius_of_sphere(chunks_inside);
            assert!((r1 as i32-r2 as i32).abs() < 2, "r1: {}, r2: {}", r1, r2);
        }
    }

    #[test]
    #[ignore]
    #[serial]
    fn chunc_generation_is_fast() {
        const TO_MILKY_WAY_CENTER: f64 = 8200.;
        const MAX_DISTANCE: f64 = TO_MILKY_WAY_CENTER / 3.;
        let params = GenerationParameters {
            observer_position_in_pc: (10., 20., 30.),
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: MAX_DISTANCE,
            chunksize_in_pc: 15.,
        };
        let start = std::time::Instant::now();
        let chunks = GalacticChunk::generate_chunks(&params);
        let duration = start.elapsed();
        println!("Generating {} chunks took {:?}", chunks.len(), duration);
        assert!(duration.as_secs_f64() < 1.);
    }

    #[test]
    #[ignore]
    #[serial]
    fn allocating_huge_amounts_of_chunks_is_possible() {
        const TO_MILKY_WAY_CENTER: f64 = 8200.;
        const MAX_DISTANCE: f64 = TO_MILKY_WAY_CENTER * 2.;
        let params = GenerationParameters {
            observer_position_in_pc: (10., 20., 30.),
            apparent_magnitude_limit: 0.,
            max_distance_in_pc: MAX_DISTANCE,
            chunksize_in_pc: 10.,
        };
        let start = std::time::Instant::now();
        let chunks = GalacticChunk::generate_chunks(&params);
        let duration = start.elapsed();
        println!("Generating {} chunks took {:?}", chunks.len(), duration);
        assert!(total_number(chunks) > MAX_ITEMS_IN_VECTOR)
    }
}
