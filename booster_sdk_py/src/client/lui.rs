use std::sync::Arc;

use booster_sdk::client::ai::{
    AsrChunk, AsrUtterance, LUI_MAX_RECOGNIZE_AUDIO_BYTES, LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS,
    LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS, LuiApiId, LuiClient, LuiRecognizeAudioRequest,
    LuiRecognizeAudioResponse, LuiSynthesizeSpeechRequest, LuiSynthesizeSpeechResponse,
    LuiTtsConfig, LuiTtsParameter, TOPIC_LUI_ASR_CHUNK,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    macros::{py_int_enum, py_param_class, py_readonly_class},
    options::rpc_options,
    runtime::wait_for_future,
    subscription::PySubscription,
    to_py_err,
};

py_int_enum!(PyLuiApiId, "LuiApiId", LuiApiId, i32, {
    START_ASR => StartAsr,
    STOP_ASR => StopAsr,
    START_TTS => StartTts,
    STOP_TTS => StopTts,
    SEND_TTS_TEXT => SendTtsText,
    SYNTHESIZE_SPEECH => SynthesizeSpeech,
    RECOGNIZE_AUDIO_ONCE => RecognizeAudioOnce,
    START_AUDIO_RECOGNIZER => StartAudioRecognizer,
    STOP_AUDIO_RECOGNIZER => StopAudioRecognizer,
    RECOGNIZE_AUDIO_IN_SESSION => RecognizeAudioInSession,
});

py_param_class!(PyLuiTtsConfig, "LuiTtsConfig", LuiTtsConfig, {
    voice_type: String = String::new(),
});

py_param_class!(PyLuiTtsParameter, "LuiTtsParameter", LuiTtsParameter, {
    text: String = String::new(),
});

py_param_class!(
    PyLuiSynthesizeSpeechRequest,
    "LuiSynthesizeSpeechRequest",
    LuiSynthesizeSpeechRequest,
    {
        text: String = String::new(),
        voice_type: String = "default".to_owned(),
        speed: f64 = 1.0,
        playback: bool = false,
    }
);

py_param_class!(
    PyLuiRecognizeAudioRequest,
    "LuiRecognizeAudioRequest",
    LuiRecognizeAudioRequest,
    {
        input_type: String = String::new(),
        file_path: String = String::new(),
        audio_base64: String = String::new(),
        sample_rate_hz: i32 = 16000,
        channels: i32 = 1,
        bits_per_sample: i32 = 16,
        format: String = "pcm_s16le".to_owned(),
    },
    {
        /// Recognize a `.wav` / `.mp3` file on the robot.
        #[staticmethod]
        fn from_file(file_path: String) -> Self {
            Self::from_rust(LuiRecognizeAudioRequest::from_file(file_path))
        }

        /// Recognize base64-encoded raw PCM (the service expects 16 kHz mono
        /// 16-bit `pcm_s16le`).
        #[staticmethod]
        #[pyo3(signature = (
            audio_base64,
            sample_rate_hz=16000,
            channels=1,
            bits_per_sample=16,
            format="pcm_s16le".to_owned(),
        ))]
        fn from_pcm_base64(
            audio_base64: String,
            sample_rate_hz: i32,
            channels: i32,
            bits_per_sample: i32,
            format: String,
        ) -> Self {
            Self::from_rust(LuiRecognizeAudioRequest::from_pcm_base64_with_format(
                audio_base64,
                sample_rate_hz,
                channels,
                bits_per_sample,
                format,
            ))
        }
    }
);

py_readonly_class!(
    PyLuiSynthesizeSpeechResponse,
    "LuiSynthesizeSpeechResponse",
    LuiSynthesizeSpeechResponse,
    {
        audio_base64: String,
        sample_rate_hz: i32,
        channels: i32,
        bits_per_sample: i32,
        format: String,
    }
);

py_readonly_class!(
    PyLuiRecognizeAudioResponse,
    "LuiRecognizeAudioResponse",
    LuiRecognizeAudioResponse,
    { text: String }
);

py_readonly_class!(PyAsrUtterance, "AsrUtterance", AsrUtterance, {
    text: String,
    definite: bool,
    start_time_ms: u32,
    end_time_ms: u32,
    speaker_id: String,
    emotion: String,
    gender: String,
    lid_lang: String,
    speech_rate: f32,
    volume_db: f32,
    additions_json: String,
});

py_readonly_class!(PyAsrChunk, "AsrChunk", AsrChunk, {
    session_id: String,
    client_id: String,
    role: String,
    text: String,
    is_final: bool,
    start_time_ms: u32,
    end_time_ms: u32,
    speaker_id: String,
    emotion: String,
    gender: String,
    lid_lang: String,
    speech_rate: f32,
    volume_db: f32,
    provider_logid: String,
    raw_json: String,
}, {
    #[getter]
    fn utterances(&self) -> Vec<PyAsrUtterance> {
        self.0.utterances.iter().cloned().map(PyAsrUtterance).collect()
    }
});

#[pyclass(module = "booster_sdk_bindings", name = "LuiClient", unsendable)]
pub struct PyLuiClient {
    client: Arc<LuiClient>,
}

#[pymethods]
impl PyLuiClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = LuiClient::with_options(options).map_err(to_py_err)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Send a raw request; the response body is discarded.
    fn send_api_request(&self, py: Python<'_>, api_id: i32, param: String) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(
            py,
            async move { client.send_api_request(api_id, &param).await },
        )
        .map_err(to_py_err)
    }

    /// Send a raw request and return the response body.
    fn send_api_request_with_response(
        &self,
        py: Python<'_>,
        api_id: i32,
        param: String,
    ) -> PyResult<String> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move {
            client.send_api_request_with_response(api_id, &param).await
        })
        .map_err(to_py_err)
    }

    /// Whether `session_id` is an ASR session started (and not yet stopped)
    /// by the client `client_id` in this process.
    #[staticmethod]
    fn is_asr_session_active(session_id: &str, client_id: &str) -> bool {
        LuiClient::is_asr_session_active(session_id, client_id)
    }

    fn start_asr(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_asr().await }).map_err(to_py_err)
    }

    fn stop_asr(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_asr().await }).map_err(to_py_err)
    }

    fn start_tts(&self, py: Python<'_>, config: PyLuiTtsConfig) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let config = config.to_rust();
        wait_for_future(py, async move { client.start_tts(&config).await }).map_err(to_py_err)
    }

    fn stop_tts(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_tts().await }).map_err(to_py_err)
    }

    fn send_tts_text(&self, py: Python<'_>, param: PyLuiTtsParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.to_rust();
        wait_for_future(py, async move { client.send_tts_text(&param).await }).map_err(to_py_err)
    }

    #[getter]
    fn client_id(&self) -> String {
        self.client.client_id().to_owned()
    }

    #[getter]
    fn current_asr_session_id(&self) -> Option<String> {
        self.client.current_asr_session_id()
    }

    #[getter]
    fn current_audio_recognizer_session_id(&self) -> Option<String> {
        self.client.current_audio_recognizer_session_id()
    }

    #[getter]
    fn current_tts_session_id(&self) -> Option<String> {
        self.client.current_tts_session_id()
    }

    fn synthesize_speech(
        &self,
        py: Python<'_>,
        req: PyLuiSynthesizeSpeechRequest,
    ) -> PyResult<PyLuiSynthesizeSpeechResponse> {
        let client = Arc::clone(&self.client);
        let req = req.to_rust();
        wait_for_future(py, async move { client.synthesize_speech(&req).await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn recognize_audio_once(
        &self,
        py: Python<'_>,
        req: PyLuiRecognizeAudioRequest,
    ) -> PyResult<PyLuiRecognizeAudioResponse> {
        let client = Arc::clone(&self.client);
        let req = req.to_rust();
        wait_for_future(py, async move { client.recognize_audio_once(&req).await })
            .map(Into::into)
            .map_err(to_py_err)
    }

    fn start_audio_recognizer(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_audio_recognizer().await }).map_err(to_py_err)
    }

    fn stop_audio_recognizer(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_audio_recognizer().await }).map_err(to_py_err)
    }

    fn recognize_audio_in_session(
        &self,
        py: Python<'_>,
        req: PyLuiRecognizeAudioRequest,
    ) -> PyResult<PyLuiRecognizeAudioResponse> {
        let client = Arc::clone(&self.client);
        let req = req.to_rust();
        wait_for_future(
            py,
            async move { client.recognize_audio_in_session(&req).await },
        )
        .map(Into::into)
        .map_err(to_py_err)
    }

    /// Subscribe to standalone ASR chunks (`rt/lui_asr_chunk`).
    fn subscribe_asr_chunk(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_asr_chunk()
            .map(PySubscription::of::<AsrChunk, PyAsrChunk>)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLuiApiId>()?;
    m.add_class::<PyLuiTtsConfig>()?;
    m.add_class::<PyLuiTtsParameter>()?;
    m.add_class::<PyLuiSynthesizeSpeechRequest>()?;
    m.add_class::<PyLuiSynthesizeSpeechResponse>()?;
    m.add_class::<PyLuiRecognizeAudioRequest>()?;
    m.add_class::<PyLuiRecognizeAudioResponse>()?;
    m.add_class::<PyAsrUtterance>()?;
    m.add_class::<PyAsrChunk>()?;
    m.add_class::<PyLuiClient>()?;
    m.add("TOPIC_LUI_ASR_CHUNK", TOPIC_LUI_ASR_CHUNK)?;
    m.add(
        "LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS",
        LUI_MAX_SYNTHESIZE_TEXT_CODE_POINTS,
    )?;
    m.add(
        "LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS",
        LUI_MAX_RECOGNIZE_AUDIO_DURATION_MS,
    )?;
    m.add(
        "LUI_MAX_RECOGNIZE_AUDIO_BYTES",
        LUI_MAX_RECOGNIZE_AUDIO_BYTES,
    )?;
    Ok(())
}
