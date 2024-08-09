#![feature(portable_simd)]

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use hedra::rast::{Block, Frame, Pixel, Point};
use hedra::{rast::TriangleRasterizerData, simd_triangle_rasterizer};

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn rasterizer(data: TriangleRasterizerData<'_, i32>) {
    simd_triangle_rasterizer!(i32, 8, data, || {})
}

struct AppData {
    window: Rc<Window>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    instant: Instant,
    frames: usize,
}

#[derive(Default)]
struct App {
    data: Option<AppData>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        let instant = Instant::now();

        self.data = Some(AppData {
            window,
            surface,
            instant,
            frames: 0,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("exiting");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let Some(data) = self.data.as_mut() else {
                    return;
                };

                let size = data.window.inner_size();
                data.surface
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = data.surface.buffer_mut().unwrap();

                buffer.fill(0);

                let rast_data = TriangleRasterizerData {
                    frame: Frame {
                        dst: &mut buffer,
                        width: size.width as usize,
                        height: size.height as usize,
                    },
                    block: Block {
                        min: Pixel { x: 25, y: 25 },
                        max: Pixel { x: 75, y: 75 },
                    },
                    list: &[[
                        Point { x: 25, y: 25 },
                        Point { x: 75, y: 25 },
                        Point { x: 75, y: 75 },
                    ]],
                };

                rasterizer(rast_data);

                buffer.present().unwrap();

                data.frames += 1;

                if data.instant.elapsed().as_secs() >= 1 {
                    println!("fps: {}", data.frames);
                    data.frames = 0;
                    data.instant = Instant::now();
                }

                data.window.request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
