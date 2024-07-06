use serde::{Deserialize, Serialize};

/// Parameters for generating a star catalogue.
///
/// Compare https://github.com/TheComamba/UrsaLumi/blob/main/Documentation/Generation_Algorithm.md#generation-parameters
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GenerationParameters {
    pub(crate) observer_position_in_pc: (f64, f64, f64),
    pub(crate) apparent_magnitude_limit: f64,
    pub(crate) max_distance_in_pc: f64,
    pub(crate) chunksize_in_pc: f64,
}
