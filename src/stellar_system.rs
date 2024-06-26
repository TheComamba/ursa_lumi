use serde::{Deserialize, Serialize};

use crate::generation_parameters::GenerationParameters;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StellarSystem {
    generation_parameters: GenerationParameters,
}

impl StellarSystem {
    pub(crate) fn new(generation_parameters: GenerationParameters) -> Self {
        StellarSystem {
            generation_parameters,
        }
    }
}
