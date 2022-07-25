use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, event::{Event, WindowEvent}};

use crate::state::WGPUState;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU!")
        .with_inner_size(LogicalSize::new(400, 400))    
        .build(&event_loop)
        .expect("Error creating window!");

    let mut state = WGPUState::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, ref event } 
            if window_id == window.id() => if !state.input(event)  {
              match event {
                WindowEvent::CloseRequested => {
                  *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(new_size) => {
                  state.resize(*new_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                  state.resize(**new_inner_size);
                }
                _ => {}
            }
          },
          Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {},
                // Perdemos o Surface => tentamos recuperar
                Err(wgpu::SurfaceError::Lost) => {
                  state.resize(state.size);
                },
                // Sem memória => deu mole
                Err(wgpu::SurfaceError::OutOfMemory) => {
                  *control_flow = ControlFlow::Exit;
                },
                Err(e) => eprint!("{:?}", e)
            }
          },
          Event::MainEventsCleared => {
            window.request_redraw();
          }
          _ => {}
        }
    });
}
