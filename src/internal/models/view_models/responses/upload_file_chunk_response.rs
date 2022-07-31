use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFileChunkResponse {
    pub file_chunk_id: String,
}
