use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder, WindowAttributes}, dpi::{LogicalSize, Position}, event::{Event, WindowEvent}};

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU!")
        .with_inner_size(LogicalSize::new(400, 400))    
        .build(&event_loop)
        .expect("Error creating window!");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, ref event } 
            if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => {}
            },
            _ => {}
        }
    });
}
