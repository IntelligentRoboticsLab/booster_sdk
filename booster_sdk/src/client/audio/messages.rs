//! DDS messages and topics published by the audio service (SDK 1.8).
//!
//! Field order follows the `m_*` members of the generated IDL headers under
//! `booster/idl/audio/`, which is the CDR serialization order. IDL enum fields
//! are carried as raw `u32` so unknown values from newer services still decode;
//! use the `*_enum()` accessors for typed views.

use rustdds::TopicKind;
use serde::{Deserialize, Serialize};

use super::types::{
    AudioCaptureFrame, AudioError, AudioErrorCategory, AudioErrorDetail, AudioEventCode, PcmFormat,
    SessionType,
};
use crate::dds::{TopicSpec, qos::qos_best_effort_keep_last};

pub const TYPE_AUDIO_PROGRESS: &str = "booster_msgs::audio::dds_::AudioProgressTopic_";
pub const TYPE_AUDIO_EVENT: &str = "booster_msgs::audio::dds_::AudioEventTopic_";
pub const TYPE_AUDIO_ERROR: &str = "booster_msgs::audio::dds_::AudioErrorTopic_";
pub const TYPE_AUDIO_CAPTURE_FRAME: &str = "booster_msgs::audio::dds_::AudioCaptureFrameTopic_";

/// The C++ SDK subscribes to `/booster/audio/progress`; on the wire (ROS 2
/// naming, like the working `rt/booster/audio/bluetooth_events`) this is
/// `rt/booster/audio/progress`.
pub const AUDIO_PROGRESS_TOPIC: &str = "rt/booster/audio/progress";
pub const AUDIO_EVENT_TOPIC: &str = "rt/booster/audio/events";
pub const AUDIO_ERROR_TOPIC: &str = "rt/booster/audio/errors";

fn convert_enum<T: TryFrom<i32>>(raw: u32) -> Option<T> {
    i32::try_from(raw).ok().and_then(|v| T::try_from(v).ok())
}

/// Common header of audio topic messages (IDL `AudioTopicHeader`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioTopicHeader {
    pub client_id: String,
    pub request_id: String,
    /// Raw [`SessionType`] value.
    pub session_type: u32,
    pub session_id: i64,
    pub timestamp_ms: i64,
}

impl AudioTopicHeader {
    #[must_use]
    pub fn session_type_enum(&self) -> Option<SessionType> {
        convert_enum(self.session_type)
    }
}

/// Playback/recording progress (IDL `AudioProgressTopic`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioProgressTopic {
    pub header: AudioTopicHeader,
    pub position_bytes: i64,
    pub duration_bytes: i64,
    /// Raw player/recorder/capture-stream state, depending on `header.session_type`.
    pub state: i32,
}

/// Lifecycle event (IDL `AudioEventTopic`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioEventTopic {
    pub header: AudioTopicHeader,
    /// Raw [`AudioEventCode`] value.
    pub event_code: u32,
    pub state: i32,
    pub detail: String,
}

impl AudioEventTopic {
    #[must_use]
    pub fn event_code_enum(&self) -> Option<AudioEventCode> {
        convert_enum(self.event_code)
    }
}

/// Service error (IDL `AudioErrorTopic`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioErrorTopic {
    pub header: AudioTopicHeader,
    pub ret_code: i32,
    /// Raw [`AudioErrorCategory`] value.
    pub error_category: u32,
    /// Raw [`AudioErrorDetail`] value.
    pub error_detail: u32,
    pub ret_msg: String,
}

impl AudioErrorTopic {
    #[must_use]
    pub fn error_category_enum(&self) -> Option<AudioErrorCategory> {
        convert_enum(self.error_category)
    }

    #[must_use]
    pub fn error_detail_enum(&self) -> Option<AudioErrorDetail> {
        convert_enum(self.error_detail)
    }
}

impl From<&AudioErrorTopic> for AudioError {
    fn from(msg: &AudioErrorTopic) -> Self {
        Self {
            ret_code: msg.ret_code,
            ret_msg: msg.ret_msg.clone(),
            error_category: msg.error_category as i32,
            error_detail: msg.error_detail as i32,
        }
    }
}

impl From<AudioErrorTopic> for AudioError {
    fn from(msg: AudioErrorTopic) -> Self {
        Self {
            ret_code: msg.ret_code,
            error_category: msg.error_category as i32,
            error_detail: msg.error_detail as i32,
            ret_msg: msg.ret_msg,
        }
    }
}

/// One PCM frame published on a capture stream's data topic
/// (IDL `AudioCaptureFrameTopic`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioCaptureFrameTopic {
    pub header: AudioTopicHeader,
    pub frame_seq: i64,
    pub raw_valid: bool,
    pub raw_sample_rate_hz: i32,
    pub raw_channels: i32,
    pub raw_bits_per_sample: i32,
    pub raw_frame_samples_per_channel: i32,
    pub raw_pcm: Vec<i16>,
    pub naec_valid: bool,
    pub naec_sample_rate_hz: i32,
    pub naec_channels: i32,
    pub naec_bits_per_sample: i32,
    pub naec_frame_samples_per_channel: i32,
    pub naec_pcm: Vec<i16>,
}

impl From<AudioCaptureFrameTopic> for AudioCaptureFrame {
    fn from(msg: AudioCaptureFrameTopic) -> Self {
        Self {
            frame_seq: msg.frame_seq,
            timestamp_ms: msg.header.timestamp_ms,
            raw_valid: msg.raw_valid,
            raw_format: PcmFormat {
                sample_rate_hz: msg.raw_sample_rate_hz,
                channels: msg.raw_channels,
                bits_per_sample: msg.raw_bits_per_sample,
            },
            raw_frame_samples_per_channel: msg.raw_frame_samples_per_channel,
            raw_pcm: msg.raw_pcm,
            naec_valid: msg.naec_valid,
            naec_format: PcmFormat {
                sample_rate_hz: msg.naec_sample_rate_hz,
                channels: msg.naec_channels,
                bits_per_sample: msg.naec_bits_per_sample,
            },
            naec_frame_samples_per_channel: msg.naec_frame_samples_per_channel,
            naec_pcm: msg.naec_pcm,
        }
    }
}

fn spec(name: String, type_name: &'static str, depth: i32) -> TopicSpec {
    // The C++ AudioManager / AudioCaptureStream create these readers with
    // `reliable = false`; a best-effort reader matches either writer kind.
    TopicSpec {
        name,
        type_name,
        qos: qos_best_effort_keep_last(depth),
        kind: TopicKind::NoKey,
    }
}

#[must_use]
pub fn audio_progress_topic() -> TopicSpec {
    spec(AUDIO_PROGRESS_TOPIC.to_owned(), TYPE_AUDIO_PROGRESS, 32)
}

#[must_use]
pub fn audio_event_topic() -> TopicSpec {
    spec(AUDIO_EVENT_TOPIC.to_owned(), TYPE_AUDIO_EVENT, 32)
}

#[must_use]
pub fn audio_error_topic() -> TopicSpec {
    spec(AUDIO_ERROR_TOPIC.to_owned(), TYPE_AUDIO_ERROR, 32)
}

/// Map a service-provided `data_topic_name` to a DDS topic name.
///
/// The C++ SDK hands the name to `ChannelSubscriber` unchanged; names the
/// audio service returns in ROS 2 form (`/booster/...`) get the `rt` prefix
/// used on the wire, names already starting with `rt/` are kept as-is.
#[must_use]
pub fn capture_frame_topic_name(data_topic_name: &str) -> String {
    let name = data_topic_name.trim();
    if name.starts_with("rt/") {
        name.to_owned()
    } else if name.starts_with('/') {
        format!("rt{name}")
    } else {
        format!("rt/{name}")
    }
}

#[must_use]
pub fn audio_capture_frame_topic(data_topic_name: &str) -> TopicSpec {
    spec(
        capture_frame_topic_name(data_topic_name),
        TYPE_AUDIO_CAPTURE_FRAME,
        64,
    )
}
