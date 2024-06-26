use serde::{Deserialize, Serialize};
use simple_si_units::base::Distance;

/// Parameters for generating a star catalogue.
/// 
/// Compare https://github.com/TheComamba/UrsaLumi/blob/main/Documentation/Generation_Algorithm.md#generation-parameters
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GenerationParameters {
    observer_position: (Distance<f64>, Distance<f64>, Distance<f64>),
    apparent_magnitude_limit: f64,
    max_distance: Distance<f64>,
}
