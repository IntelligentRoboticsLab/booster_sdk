use std::sync::Arc;

use booster_sdk::client::ai::{
    AiApiId, AiClient, AsrConfig, LlmConfig, SpeakParameter, StartAiChatParameter, Subtitle,
    TOPIC_AI_SUBTITLE, TtsConfig,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{
    client::device::json_err,
    macros::{py_int_enum, py_param_class, py_readonly_class},
    options::rpc_options,
    runtime::wait_for_future,
    subscription::PySubscription,
    to_py_err,
};

py_int_enum!(PyAiApiId, "AiApiId", AiApiId, i32, {
    START_AI_CHAT => StartAiChat,
    STOP_AI_CHAT => StopAiChat,
    SPEAK => Speak,
    START_FACE_TRACKING => StartFaceTracking,
    STOP_FACE_TRACKING => StopFaceTracking,
});

py_param_class!(PyTtsConfig, "TtsConfig", TtsConfig, {
    voice_type: String = String::new(),
    ignore_bracket_text: Vec<i8> = Vec::new(),
});

py_param_class!(PyLlmConfig, "LlmConfig", LlmConfig, {
    system_prompt: String = String::new(),
    welcome_msg: String = String::new(),
    prompt_name: String = String::new(),
});

py_param_class!(PyAsrConfig, "AsrConfig", AsrConfig, {
    interrupt_speech_duration: i32 = 0,
    interrupt_keywords: Vec<String> = Vec::new(),
});

/// Parameters for `AiClient.start_ai_chat`. The nested configs are shared
/// objects: `param.llm_config.prompt_name = "x"` updates `param`.
#[pyclass(
    module = "booster_sdk_bindings",
    name = "StartAiChatParameter",
    get_all,
    set_all
)]
pub struct PyStartAiChatParameter {
    persona_id: Option<String>,
    interrupt_mode: bool,
    asr_config: Py<PyAsrConfig>,
    llm_config: Py<PyLlmConfig>,
    tts_config: Py<PyTtsConfig>,
    enable_face_tracking: bool,
}

impl PyStartAiChatParameter {
    fn to_rust(&self, py: Python<'_>) -> StartAiChatParameter {
        StartAiChatParameter {
            persona_id: self.persona_id.clone(),
            interrupt_mode: self.interrupt_mode,
            asr_config: self.asr_config.borrow(py).to_rust(),
            llm_config: self.llm_config.borrow(py).to_rust(),
            tts_config: self.tts_config.borrow(py).to_rust(),
            enable_face_tracking: self.enable_face_tracking,
        }
    }

    fn from_rust(py: Python<'_>, value: StartAiChatParameter) -> PyResult<Self> {
        Ok(Self {
            persona_id: value.persona_id,
            interrupt_mode: value.interrupt_mode,
            asr_config: Py::new(py, PyAsrConfig::from_rust(value.asr_config))?,
            llm_config: Py::new(py, PyLlmConfig::from_rust(value.llm_config))?,
            tts_config: Py::new(py, PyTtsConfig::from_rust(value.tts_config))?,
            enable_face_tracking: value.enable_face_tracking,
        })
    }
}

#[pymethods]
impl PyStartAiChatParameter {
    #[new]
    #[pyo3(signature = (
        interrupt_mode=false,
        asr_config=None,
        llm_config=None,
        tts_config=None,
        enable_face_tracking=false,
        persona_id=None,
    ))]
    fn new(
        py: Python<'_>,
        interrupt_mode: bool,
        asr_config: Option<Py<PyAsrConfig>>,
        llm_config: Option<Py<PyLlmConfig>>,
        tts_config: Option<Py<PyTtsConfig>>,
        enable_face_tracking: bool,
        persona_id: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            persona_id,
            interrupt_mode,
            asr_config: asr_config.map_or_else(|| Py::new(py, PyAsrConfig::default()), Ok)?,
            llm_config: llm_config.map_or_else(|| Py::new(py, PyLlmConfig::default()), Ok)?,
            tts_config: tts_config.map_or_else(|| Py::new(py, PyTtsConfig::default()), Ok)?,
            enable_face_tracking,
        })
    }

    /// Serialize to the JSON body sent to the service.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        serde_json::to_string(&self.to_rust(py)).map_err(json_err)
    }

    /// Parse from a JSON body.
    #[staticmethod]
    fn from_json(py: Python<'_>, json: &str) -> PyResult<Self> {
        Self::from_rust(py, serde_json::from_str(json).map_err(json_err)?)
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        format!("{:?}", self.to_rust(py))
    }
}

py_param_class!(PySpeakParameter, "SpeakParameter", SpeakParameter, {
    msg: String = String::new(),
});

py_readonly_class!(PySubtitle, "Subtitle", Subtitle, {
    magic_number: String,
    text: String,
    language: String,
    user_id: String,
    seq: i32,
    definite: bool,
    paragraph: bool,
    round_id: i32,
});

#[pyclass(module = "booster_sdk_bindings", name = "AiClient", unsendable)]
pub struct PyAiClient {
    client: Arc<AiClient>,
}

#[pymethods]
impl PyAiClient {
    #[new]
    #[pyo3(signature = (startup_wait_sec=None, *, robot_name=None, network=None, domain_id=None))]
    fn new(
        startup_wait_sec: Option<f64>,
        robot_name: Option<String>,
        network: Option<&Bound<'_, PyAny>>,
        domain_id: Option<u16>,
    ) -> PyResult<Self> {
        let options = rpc_options(startup_wait_sec, robot_name, network, domain_id)?;
        let client = AiClient::with_options(options).map_err(to_py_err)?;
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

    fn start_ai_chat(
        &self,
        py: Python<'_>,
        param: PyRef<'_, PyStartAiChatParameter>,
    ) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.to_rust(py);
        wait_for_future(py, async move { client.start_ai_chat(&param).await }).map_err(to_py_err)
    }

    fn stop_ai_chat(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_ai_chat().await }).map_err(to_py_err)
    }

    fn speak(&self, py: Python<'_>, param: PyRef<'_, PySpeakParameter>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.to_rust();
        wait_for_future(py, async move { client.speak(&param).await }).map_err(to_py_err)
    }

    fn start_face_tracking(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.start_face_tracking().await }).map_err(to_py_err)
    }

    fn stop_face_tracking(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_face_tracking().await }).map_err(to_py_err)
    }

    /// Subscribe to AI chat subtitles (`rt/ai_subtitle`).
    fn subscribe_subtitle(&self) -> PyResult<PySubscription> {
        self.client
            .subscribe_subtitle()
            .map(PySubscription::of::<Subtitle, PySubtitle>)
            .map_err(to_py_err)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAiApiId>()?;
    m.add_class::<PyTtsConfig>()?;
    m.add_class::<PyLlmConfig>()?;
    m.add_class::<PyAsrConfig>()?;
    m.add_class::<PyStartAiChatParameter>()?;
    m.add_class::<PySpeakParameter>()?;
    m.add_class::<PySubtitle>()?;
    m.add_class::<PyAiClient>()?;
    m.add("TOPIC_AI_SUBTITLE", TOPIC_AI_SUBTITLE)?;
    Ok(())
}
