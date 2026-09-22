use xcap::Monitor;
use tokio::time::{sleep, Duration};
use protocol::{InputEvent, MouseButton};

mod input;
use input::InputExecutor;

#[tokio::main]
async fn main() {
    println!("RemoteLinkDesk - Windows Host Native Application");
    
    let monitors = Monitor::all().unwrap_or_else(|_| vec![]);
    if monitors.is_empty() {
        println!("No monitors detected.");
        return;
    }
    
    let primary = &monitors[0];
    let screen_width = primary.width();
    let screen_height = primary.height();
    
    println!("Capturing monitor: {} ({}x{})", primary.name(), screen_width, screen_height);

    let mut executor = InputExecutor::new();

    // Simulated main loop for Phase 5 integration
    loop {
        // 1. Capture Screen Frame
        // let image = primary.capture_image().unwrap();
        
        // 2. Transmit Screen Frame (Omitted)
        
        // 3. Receive Input Event (Simulated click in center of screen)
        // In reality, this would come from the tokio TCP stream
        // let simulated_event = InputEvent::MouseMove { x: 0.5, y: 0.5 };
        // executor.execute_event(simulated_event, screen_width, screen_height);
        
        sleep(Duration::from_millis(16)).await;
    }
}
