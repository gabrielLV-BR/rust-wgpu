use wgpu::{util::DeviceExt, BlendState, PrimitiveState, RequestAdapterOptions, TextureUsages, VertexBufferLayout};
use winit::event::WindowEvent;

#[repr(C)]
// Must derive for `bytemuck` to work
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexBufferLayout {
            // Stride: total size of Vertex
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute{
                    // Offset: size difference between this attribute 
                    // and start of vertex
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
      position: [0.5, -0.5, 0.0],
      color: [0.0, 0.0, 1.0],
  },
];

const INDICES: &[u16 ; 6] = &[
  2, 1, 0,
  0, 3, 2
];

pub struct WGPUState {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}

impl WGPUState {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        // Instance é um handle pra GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        // Surface é onde iremos desenhar na janela
        let surface = unsafe { instance.create_surface(&window) };

        println!("Surface: {:?}", surface);
        // Adapter é uma referência lógica à GPU, nos permite definir algumas preferências
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::HighPerformance,
            })
            .await
            .expect("Erro ao criar ADAPTER!");

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
            .expect("Erro ao criar DEVICE/QUEUE!");

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
            present_mode: surface
                .get_supported_modes(&adapter)
                .iter()
                .find(|p| **p == wgpu::PresentMode::Mailbox)
                .unwrap_or(&wgpu::PresentMode::Fifo)
                .to_owned(),
        };

        // Configura nosso surface com nossas opções
        surface.configure(&device, &config);

        // Poderia ser resumido em
        // let shader = include_wgsl!("resources/shaders/basic.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Basic shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("resources/shaders/basic.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Basic render pipeline"),
            primitive: PrimitiveState {
                //  Topologia
                topology: wgpu::PrimitiveTopology::TriangleList,
                //  Só util quando topologia é Strip com índices
                strip_index_format: None,
                // Frente -> vértices em Counter ClockWise
                front_face: wgpu::FrontFace::Ccw,
                // Esconder os de trás
                cull_mode: Some(wgpu::Face::Back),
                // Preencher o triângulo
                polygon_mode: wgpu::PolygonMode::Fill,
                // Não realiza o clipping, possível extra-processando
                // de fragmentos que serão descartados
                unclipped_depth: false,
                // Se true, a rasterização é mais conservadora
                conservative: false,
            },
            // O efeito que o passe vai ter no buffer de depth/stencil
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                // Quantos samples calculados por pixel (MSAA)
                count: 1,
                // bitmask de samples ativos (!0 => 1111..., ou seja, todos)
                mask: !0,
                // também não entendi, tem a haver com anti-aliasing
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &[
                    // Set layout to use
                    Vertex::describe()
                ],
                entry_point: "vs_main",
                module: &shader,
            },
            fragment: Some(wgpu::FragmentState {
                entry_point: "fs_main",
                module: &shader,
                targets: &[Some(wgpu::ColorTargetState {
                    blend: Some(BlendState::ALPHA_BLENDING),
                    format: config.format,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("VertexBuffer"),
            // Cast VERTICES to &[u8]
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
          label: Some("IndexBuffer"),
          contents: bytemuck::cast_slice(INDICES),
          usage: wgpu::BufferUsages::INDEX
        });

        Self {
            device,
            queue,
            size,
            surface,
            surface_config: config,
            render_pipeline,
            vertex_buffer,
            index_buffer
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

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Pega texture no Surface para desenhar
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // CommandEncoder: tipo o CommandPool do Vulkan
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        {
            // Só uma referência, o RenderPass já está salvo no encoder
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Screen Clearing"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.5,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            // Set vertex to use
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
            // render_pass.draw(0..VERTICES.len() as u32, 0..1);

        }

        self.queue.submit(std::iter::once(encoder.finish()));
        // Apresenta na tela
        output.present();

        Ok(())
    }
}
