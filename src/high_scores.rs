use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::difficulty::Difficulty;
use crate::grid_size::GridSize;

const MAX_ENTRIES: usize = 5;
const SAVE_FILE: &str = "high_scores.bin";

#[derive(Serialize, Deserialize, Clone)]
pub struct HighScoreEntry {
    pub score: u32,
}

#[derive(Serialize, Deserialize)]
struct HighScoresPayloadV1 {
    entries: HashMap<(GridSize, Difficulty), Vec<HighScoreEntry>>,
}

#[derive(Serialize, Deserialize)]
struct VersionedHighScores {
    version: u8,
    data: Vec<u8>,
}

pub struct HighScores {
    payload: HighScoresPayloadV1,
    save_path: Option<PathBuf>,
}

impl HighScores {
    pub fn load() -> Self {
        let save_path = Self::resolve_save_path();
        let payload = save_path
            .as_ref()
            .and_then(|p| fs::read(p).ok())
            .and_then(|bytes| postcard::from_bytes::<VersionedHighScores>(&bytes).ok())
            .and_then(|envelope| match envelope.version {
                1 => postcard::from_bytes::<HighScoresPayloadV1>(&envelope.data).ok(),
                v => {
                    eprintln!("high_scores: unknown version {v}");
                    None
                }
            })
            .unwrap_or_else(|| HighScoresPayloadV1 {
                entries: HashMap::new(),
            });

        Self { payload, save_path }
    }

    pub fn record(&mut self, grid_size: GridSize, difficulty: Difficulty, score: u32) {
        let bucket = self
            .payload
            .entries
            .entry((grid_size, difficulty))
            .or_default();
        bucket.push(HighScoreEntry { score });
        bucket.sort_unstable_by(|a, b| b.score.cmp(&a.score));
        bucket.truncate(MAX_ENTRIES);
        self.persist();
    }

    pub fn get_scores_for(&self, grid_size: GridSize, difficulty: Difficulty) -> &[HighScoreEntry] {
        self.payload
            .entries
            .get(&(grid_size, difficulty))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    fn persist(&self) {
        let Some(path) = &self.save_path else { return };
        let inner = match postcard::to_stdvec(&self.payload) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("high_scores: serialize payload: {e}");
                return;
            }
        };
        let envelope = VersionedHighScores {
            version: 1,
            data: inner,
        };
        match postcard::to_stdvec(&envelope) {
            Ok(bytes) => {
                let _ = fs::write(path, bytes);
            }
            Err(e) => eprintln!("high_scores: serialize envelope: {e}"),
        }
    }

    fn resolve_save_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "xapphire13", "bleak-blocks").map(|dirs| {
            let data_dir = dirs.data_dir().to_path_buf();
            let _ = fs::create_dir_all(&data_dir);
            data_dir.join(SAVE_FILE)
        })
    }
}
