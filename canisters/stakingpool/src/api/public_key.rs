use wallet::{ecdsa, utils::principal_to_derivation_path};

use crate::{domain::Metadata, error::StakingError};

pub async fn serve(metadata: Metadata) -> Result<Vec<u8>, StakingError> {
    ecdsa::public_key(
        principal_to_derivation_path(metadata.owner),
        metadata.ecdsa_key_id,
        None,
    )
    .await
    .map_err(|e| e.into())
}
