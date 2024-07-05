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
    pub(crate) fn generate_chunks(generation_parameters: &GenerationParameters) -> Vec<Self> {
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

    fn create_chunks_in_sphere(
        generation_parameters: &GenerationParameters,
        chunk_numbers_first_octant: Vec<(usize, usize, usize)>,
    ) -> Vec<Self> {
        let origin_chunk = Self::origin_chunk(generation_parameters);
        let (x0, y0, z0) = origin_chunk.corner;

        let mut chunks: Vec<Self> = Vec::with_capacity(chunk_numbers_first_octant.len() * 8);
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

fn chunk_numbers_in_first_octant(n: usize) -> Vec<(usize, usize, usize)> {
    let mut chunks_first_octant = Vec::new();
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

fn number_of_chunks_along_axis(generation_parameters: &GenerationParameters) -> usize {
    (generation_parameters.max_distance_in_pc / generation_parameters.chunksize_in_pc).ceil()
        as usize
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
        assert_eq!(chunk_numbers_in_first_octant(1), vec![(0, 0, 0)]);
        assert_eq!(
            chunk_numbers_in_first_octant(2),
            vec![
                (0, 0, 0),
                (0, 0, 1),
                (0, 1, 0),
                (1, 0, 0),
                (0, 1, 1),
                (1, 0, 1),
                (1, 1, 0),
                (1, 1, 1)
            ]
        );
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
        println!(
            "Generating {} chunks took {:?}, or {:?} per chunk",
            chunks.len(),
            duration,
            duration / chunks.len() as u32
        );
        assert!(duration.as_secs_f64() < 1.);
    }
}
