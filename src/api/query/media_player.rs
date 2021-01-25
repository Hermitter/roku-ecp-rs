use super::{from_str, Deserialize, Device, Error};

impl Device {
    /// Information on the current media player state. This includes the current stream segment and position of the content being played, the running time of the content, audio format, and buffering.
    pub async fn media_player(&self) -> Result<MediaPlayer, Error> {
        let response = self
            .http
            .get(self.url.join("query/media-player")?)
            .recv_string()
            .await?;

        Ok(from_str(&response)?)
    }
}

/// Information on the current state of the media player.
#[derive(Debug, Deserialize)]
pub struct MediaPlayer {
    pub error: String,
    pub state: String,
    pub format: Option<Format>,
    pub buffering: Option<Buffering>,
    pub new_stream: Option<NewStream>,
    pub position: Option<String>,
    pub duration: Option<String>,
    pub is_live: Option<String>,
    pub runtime: Option<String>,
    pub stream_segment: Option<StreamSegment>,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub audio: String,
    pub captions: String,
    pub container: Option<String>,
    pub drm: String,
    pub video: String,
    pub video_res: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Buffering {
    pub current: String,
    pub max: String,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct NewStream {
    pub speed: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamSegment {
    pub bitrate: u32,
    pub media_sequence: u32,
    pub segment_type: String,
    pub time: u32,
}
