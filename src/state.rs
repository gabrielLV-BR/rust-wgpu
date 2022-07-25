use wgpu::{Device, RequestAdapterOptions, TextureUsages};
use winit::event::WindowEvent;

struct State {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // Instance é um handle pra GPU
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        // Surface é onde iremos desenhar na janela
        let surface = unsafe { instance.create_surface(&window) };
        // Adapter é uma referência lógica à GPU, nos permite definir algumas preferências
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: true,
                power_preference: wgpu::PowerPreference::HighPerformance,
            })
            .await
            .expect("Erro ao requerir adaptador: requerimentos não foram satisfeitos!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    // Features: que funcionalidades nós desejamos que a GPU providencie
                    features: wgpu::Features::empty(),
                    // Limits: limita nosso acesso à certas funcionalidades a fim de
                    // atingir mais dispositivos
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Erro ao criar Device e Queue");

        // Define opções de como o wgpu deve criar o Surface
        let config = wgpu::SurfaceConfiguration {
            // Para quê vamos usar a Surface? RENDERizar
            usage: TextureUsages::RENDER_ATTACHMENT,
            // Formato da textura (R8G8B8A8, etc.)
            format: surface.get_supported_formats(&adapter)[0],
            // Tamanho
            width: size.width,
            height: size.height,
            // PresentMode: V-Sync, TripleBuffering, direto...
            present_mode: surface.get_supported_modes(&adapter)
            .iter()
            .find(|p| **p == wgpu::PresentMode::Mailbox)
            .unwrap_or(&wgpu::PresentMode::Fifo)
            .to_owned(),
        };

        // Configura nosso surface com nossas opções
        surface.configure(&device, &config);

        Self {
          device,
          queue,
          size,
          surface,
          surface_config: config
        }        
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
