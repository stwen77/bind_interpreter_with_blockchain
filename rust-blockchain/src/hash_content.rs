//! Hashed content of every chain block.

use time;

#[derive(Serialize, Deserialize, Clone)]
pub struct HashContent {
    timestamp: i64,
    data: Vec<u8>,
}

impl HashContent {
    /// Creates a brand new hash content.
    ///
    /// Args:
    ///
    /// `data` - the data to store into the block hash content
    ///
    /// Returns:
    ///
    /// hash content with current timestamp and given data
    pub fn new(data: Vec<u8>) -> HashContent {
        HashContent {
            timestamp: time::now_utc().to_timespec().sec,
            data: data,
        }
    }

    /// Getter of the timestamp.
    ///
    /// Returns:
    ///
    /// block creation timestamp
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Getter of the data.
    ///
    /// Returns:
    ///
    /// block data
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
