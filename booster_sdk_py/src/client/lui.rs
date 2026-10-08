use std::sync::Arc;

use booster_sdk::client::ai::{
    LuiClient, LuiRecognizeAudioRequest, LuiRecognizeAudioResponse, LuiSynthesizeSpeechRequest,
    LuiSynthesizeSpeechResponse, LuiTtsConfig, LuiTtsParameter,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, startup_wait_from_seconds, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "LuiTtsConfig")]
#[derive(Clone)]
pub struct PyLuiTtsConfig(LuiTtsConfig);

#[pymethods]
impl PyLuiTtsConfig {
    #[new]
    fn new(voice_type: String) -> Self {
        Self(LuiTtsConfig { voice_type })
    }

    #[getter]
    fn voice_type(&self) -> String {
        self.0.voice_type.clone()
    }
}

impl From<PyLuiTtsConfig> for LuiTtsConfig {
    fn from(value: PyLuiTtsConfig) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiTtsParameter")]
#[derive(Clone)]
pub struct PyLuiTtsParameter(LuiTtsParameter);

#[pymethods]
impl PyLuiTtsParameter {
    #[new]
    fn new(text: String) -> Self {
        Self(LuiTtsParameter { text })
    }

    #[getter]
    fn text(&self) -> String {
        self.0.text.clone()
    }
}

impl From<PyLuiTtsParameter> for LuiTtsParameter {
    fn from(value: PyLuiTtsParameter) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiSynthesizeSpeechRequest")]
#[derive(Clone)]
pub struct PyLuiSynthesizeSpeechRequest(LuiSynthesizeSpeechRequest);

#[pymethods]
impl PyLuiSynthesizeSpeechRequest {
    #[new]
    #[pyo3(signature = (text, voice_type="default".to_owned(), speed=1.0, playback=false))]
    fn new(text: String, voice_type: String, speed: f64, playback: bool) -> Self {
        Self(LuiSynthesizeSpeechRequest {
            text,
            voice_type,
            speed,
            playback,
        })
    }

    #[getter]
    fn text(&self) -> String {
        self.0.text.clone()
    }

    #[getter]
    fn voice_type(&self) -> String {
        self.0.voice_type.clone()
    }

    #[getter]
    fn speed(&self) -> f64 {
        self.0.speed
    }

    #[getter]
    fn playback(&self) -> bool {
        self.0.playback
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiSynthesizeSpeechResponse")]
#[derive(Clone)]
pub struct PyLuiSynthesizeSpeechResponse(LuiSynthesizeSpeechResponse);

#[pymethods]
impl PyLuiSynthesizeSpeechResponse {
    #[getter]
    fn audio_base64(&self) -> String {
        self.0.audio_base64.clone()
    }

    #[getter]
    fn sample_rate_hz(&self) -> i32 {
        self.0.sample_rate_hz
    }

    #[getter]
    fn channels(&self) -> i32 {
        self.0.channels
    }

    #[getter]
    fn bits_per_sample(&self) -> i32 {
        self.0.bits_per_sample
    }

    #[getter]
    fn format(&self) -> String {
        self.0.format.clone()
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiRecognizeAudioRequest")]
#[derive(Clone)]
pub struct PyLuiRecognizeAudioRequest(LuiRecognizeAudioRequest);

#[pymethods]
impl PyLuiRecognizeAudioRequest {
    #[staticmethod]
    fn from_file(file_path: String) -> Self {
        Self(LuiRecognizeAudioRequest::from_file(file_path))
    }

    #[staticmethod]
    fn from_pcm_base64(audio_base64: String) -> Self {
        Self(LuiRecognizeAudioRequest::from_pcm_base64(audio_base64))
    }

    #[getter]
    fn input_type(&self) -> String {
        self.0.input_type.clone()
    }

    #[getter]
    fn file_path(&self) -> String {
        self.0.file_path.clone()
    }

    #[getter]
    fn sample_rate_hz(&self) -> i32 {
        self.0.sample_rate_hz
    }

    #[getter]
    fn channels(&self) -> i32 {
        self.0.channels
    }

    #[getter]
    fn bits_per_sample(&self) -> i32 {
        self.0.bits_per_sample
    }

    #[getter]
    fn format(&self) -> String {
        self.0.format.clone()
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiRecognizeAudioResponse")]
#[derive(Clone)]
pub struct PyLuiRecognizeAudioResponse(LuiRecognizeAudioResponse);

#[pymethods]
impl PyLuiRecognizeAudioResponse {
    #[getter]
    fn text(&self) -> String {
        self.0.text.clone()
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LuiClient", unsendable)]
pub struct PyLuiClient {
    client: Arc<LuiClient>,
}

#[pymethods]
impl PyLuiClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None))]
    fn new(startup_wait_sec: Option<f64>) -> PyResult<Self> {
        let startup_wait = startup_wait_from_seconds(startup_wait_sec)?;
        let client = match startup_wait {
            Some(wait) => LuiClient::with_startup_wait(wait),
            None => LuiClient::new(),
        }
        .map_err(to_py_err)?;

        Ok(Self {
            client: Arc::new(client),
        })
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
        let config = config.into();
        wait_for_future(py, async move { client.start_tts(&config).await }).map_err(to_py_err)
    }

    fn stop_tts(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_tts().await }).map_err(to_py_err)
    }

    fn send_tts_text(&self, py: Python<'_>, param: PyLuiTtsParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
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
        wait_for_future(py, async move { client.synthesize_speech(&req.0).await })
            .map(PyLuiSynthesizeSpeechResponse)
            .map_err(to_py_err)
    }

    fn recognize_audio_once(
        &self,
        py: Python<'_>,
        req: PyLuiRecognizeAudioRequest,
    ) -> PyResult<PyLuiRecognizeAudioResponse> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.recognize_audio_once(&req.0).await })
            .map(PyLuiRecognizeAudioResponse)
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
        wait_for_future(py, async move {
            client.recognize_audio_in_session(&req.0).await
        })
        .map(PyLuiRecognizeAudioResponse)
        .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLuiTtsConfig>()?;
    m.add_class::<PyLuiTtsParameter>()?;
    m.add_class::<PyLuiSynthesizeSpeechRequest>()?;
    m.add_class::<PyLuiSynthesizeSpeechResponse>()?;
    m.add_class::<PyLuiRecognizeAudioRequest>()?;
    m.add_class::<PyLuiRecognizeAudioResponse>()?;
    m.add_class::<PyLuiClient>()?;
    Ok(())
}
