use enigo::{
    Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings
};
use protocol::{InputEvent, MouseButton};

/// Responsible for executing native OS inputs
pub struct InputExecutor {
    enigo: Enigo,
}

impl InputExecutor {
    pub fn new() -> Self {
        // Initialize enigo for simulating hardware events
        let enigo = Enigo::new(&Settings::default()).unwrap();
        Self { enigo }
    }

    /// Process an incoming input event from the remote client
    pub fn execute_event(&mut self, event: InputEvent, screen_width: u32, screen_height: u32) {
        match event {
            InputEvent::MouseMove { x, y } => {
                // Map normalized coordinates (0.0 - 1.0) to absolute screen pixels
                let abs_x = (x * screen_width as f32) as i32;
                let abs_y = (y * screen_height as f32) as i32;
                let _ = self.enigo.move_mouse(abs_x, abs_y, Coordinate::Abs);
            }
            InputEvent::MouseClick { button, down } => {
                let enigo_button = match button {
                    MouseButton::Left => Button::Left,
                    MouseButton::Right => Button::Right,
                    MouseButton::Middle => Button::Middle,
                };
                
                let direction = if down { Direction::Press } else { Direction::Release };
                let _ = self.enigo.button(enigo_button, direction);
            }
            InputEvent::KeyPress { key_code: _, down: _ } => {
                // In a complete implementation, map the protocol key_code to Enigo's Key enum
                // e.g. self.enigo.key(Key::Layout(char), direction)
                println!("Key press execution omitted for brevity");
            }
        }
    }
}
