use wgpu::{Device, RequestAdapterOptions, TextureUsages};
use winit::event::WindowEvent;

pub struct WGPUState {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl WGPUState {
    pub async fn new(window: &winit::window::Window) -> Self {
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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
      if new_size.width > 0 && new_size.height > 0 {
        self.size = new_size;
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;
        self.surface.configure(&self.device, &self.surface_config);
      } 
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
      false
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
      // Pega texture no Surface para desenhar
      let output = self.surface.get_current_texture()?;

      let view = output.texture.create_view(
        &wgpu::TextureViewDescriptor::default()
      );

      // CommandEncoder: tipo o CommandPool do Vulkan
      let mut encoder = self.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor::default()
      );

      {
        // Só uma referência, o RenderPass já está salvo no encoder
        let _render_pass = encoder.begin_render_pass(
          &wgpu::RenderPassDescriptor {
            label: Some("Screen Clearing"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
              view: &view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.2,
                  g: 0.5,
                  b: 0.8,
                  a: 1.0
                }),
                store: false
              },
            })],
            depth_stencil_attachment: None
          }
        );
      }

      self.queue.submit(std::iter::once(encoder.finish()));
      // Apresenta na tela
      output.present();

      Ok(())
    }
}
