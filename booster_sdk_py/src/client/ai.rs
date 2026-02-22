use std::sync::Arc;

use booster_sdk::client::ai::{
    AiClient, AsrConfig, LlmConfig, SpeakParameter, StartAiChatParameter, TtsConfig,
};
use pyo3::{Bound, prelude::*, types::PyModule};

use crate::{runtime::wait_for_future, to_py_err};

#[pyclass(module = "booster_sdk_bindings", name = "TtsConfig")]
#[derive(Clone)]
pub struct PyTtsConfig(TtsConfig);

#[pymethods]
impl PyTtsConfig {
    #[new]
    fn new(voice_type: String, ignore_bracket_text: Vec<i8>) -> Self {
        Self(TtsConfig {
            voice_type,
            ignore_bracket_text,
        })
    }

    #[getter]
    fn voice_type(&self) -> String {
        self.0.voice_type.clone()
    }

    #[getter]
    fn ignore_bracket_text(&self) -> Vec<i8> {
        self.0.ignore_bracket_text.clone()
    }
}

impl From<PyTtsConfig> for TtsConfig {
    fn from(value: PyTtsConfig) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "LlmConfig")]
#[derive(Clone)]
pub struct PyLlmConfig(LlmConfig);

#[pymethods]
impl PyLlmConfig {
    #[new]
    fn new(system_prompt: String, welcome_msg: String, prompt_name: String) -> Self {
        Self(LlmConfig {
            system_prompt,
            welcome_msg,
            prompt_name,
        })
    }

    #[getter]
    fn system_prompt(&self) -> String {
        self.0.system_prompt.clone()
    }

    #[getter]
    fn welcome_msg(&self) -> String {
        self.0.welcome_msg.clone()
    }

    #[getter]
    fn prompt_name(&self) -> String {
        self.0.prompt_name.clone()
    }
}

impl From<PyLlmConfig> for LlmConfig {
    fn from(value: PyLlmConfig) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AsrConfig")]
#[derive(Clone)]
pub struct PyAsrConfig(AsrConfig);

#[pymethods]
impl PyAsrConfig {
    #[new]
    fn new(interrupt_speech_duration: i32, interrupt_keywords: Vec<String>) -> Self {
        Self(AsrConfig {
            interrupt_speech_duration,
            interrupt_keywords,
        })
    }

    #[getter]
    fn interrupt_speech_duration(&self) -> i32 {
        self.0.interrupt_speech_duration
    }

    #[getter]
    fn interrupt_keywords(&self) -> Vec<String> {
        self.0.interrupt_keywords.clone()
    }
}

impl From<PyAsrConfig> for AsrConfig {
    fn from(value: PyAsrConfig) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "StartAiChatParameter")]
#[derive(Clone)]
pub struct PyStartAiChatParameter(StartAiChatParameter);

#[pymethods]
impl PyStartAiChatParameter {
    #[new]
    fn new(
        interrupt_mode: bool,
        asr_config: PyAsrConfig,
        llm_config: PyLlmConfig,
        tts_config: PyTtsConfig,
        enable_face_tracking: bool,
    ) -> Self {
        Self(StartAiChatParameter {
            interrupt_mode,
            asr_config: asr_config.into(),
            llm_config: llm_config.into(),
            tts_config: tts_config.into(),
            enable_face_tracking,
        })
    }

    #[getter]
    fn interrupt_mode(&self) -> bool {
        self.0.interrupt_mode
    }

    #[getter]
    fn asr_config(&self) -> PyAsrConfig {
        PyAsrConfig(self.0.asr_config.clone())
    }

    #[getter]
    fn llm_config(&self) -> PyLlmConfig {
        PyLlmConfig(self.0.llm_config.clone())
    }

    #[getter]
    fn tts_config(&self) -> PyTtsConfig {
        PyTtsConfig(self.0.tts_config.clone())
    }

    #[getter]
    fn enable_face_tracking(&self) -> bool {
        self.0.enable_face_tracking
    }
}

impl From<PyStartAiChatParameter> for StartAiChatParameter {
    fn from(value: PyStartAiChatParameter) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "SpeakParameter")]
#[derive(Clone)]
pub struct PySpeakParameter(SpeakParameter);

#[pymethods]
impl PySpeakParameter {
    #[new]
    fn new(msg: String) -> Self {
        Self(SpeakParameter { msg })
    }

    #[getter]
    fn msg(&self) -> String {
        self.0.msg.clone()
    }
}

impl From<PySpeakParameter> for SpeakParameter {
    fn from(value: PySpeakParameter) -> Self {
        value.0
    }
}

#[pyclass(module = "booster_sdk_bindings", name = "AiClient", unsendable)]
pub struct PyAiClient {
    client: Arc<AiClient>,
}

#[pymethods]
impl PyAiClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            client: Arc::new(AiClient::new().map_err(to_py_err)?),
        })
    }

    fn start_ai_chat(&self, py: Python<'_>, param: PyStartAiChatParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
        wait_for_future(py, async move { client.start_ai_chat(&param).await }).map_err(to_py_err)
    }

    fn stop_ai_chat(&self, py: Python<'_>) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        wait_for_future(py, async move { client.stop_ai_chat().await }).map_err(to_py_err)
    }

    fn speak(&self, py: Python<'_>, param: PySpeakParameter) -> PyResult<()> {
        let client = Arc::clone(&self.client);
        let param = param.into();
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
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTtsConfig>()?;
    m.add_class::<PyLlmConfig>()?;
    m.add_class::<PyAsrConfig>()?;
    m.add_class::<PyStartAiChatParameter>()?;
    m.add_class::<PySpeakParameter>()?;
    m.add_class::<PyAiClient>()?;
    Ok(())
}
