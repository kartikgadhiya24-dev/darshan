use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    /// A frame of the screen sent from Host to Client
    ScreenFrame(ScreenFrame),
    /// An input event sent from Client to Host
    InputEvent(InputEvent),
    /// Connection keep-alive ping
    Ping,
    /// Connection keep-alive pong
    Pong,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScreenFrame {
    pub width: u32,
    pub height: u32,
    pub is_keyframe: bool,
    /// Encoded video data (e.g., JPEG or H264 bytes)
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputEvent {
    MouseMove { x: f32, y: f32 }, // Normalized coordinates 0.0 - 1.0
    MouseClick { button: MouseButton, down: bool },
    KeyPress { key_code: u32, down: bool },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}
