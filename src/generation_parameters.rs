use serde::{Deserialize, Serialize};

/// Parameters for generating a star catalogue.
///
/// Compare https://github.com/TheComamba/UrsaLumi/blob/main/Documentation/Generation_Algorithm.md#generation-parameters
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GenerationParameters {
    observer_position_in_pc: (f64, f64, f64),
    apparent_magnitude_limit: f64,
    max_distance_in_pc: f64,
}
