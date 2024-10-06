#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
	event::*,
	event_loop::EventLoop,
	keyboard::{KeyCode, PhysicalKey},
	window::{Window, WindowBuilder}
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
	cfg_if::cfg_if! {
		if #[cfg(target_arch = "wasm32")] {
			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
			console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
		} else {
			env_logger::init();
		}
	}
	log::info!("Started running and initalized logger.");

	let event_loop = EventLoop::new().unwrap();
	let window = WindowBuilder::new().build(&event_loop).unwrap();

	#[cfg(target_arch = "wasm32")]
	{
		// Winit prevents sizing with CSS, so we have to set the size manually when on
		// the web.
		use winit::dpi::PhysicalSize;
		let _ = window.request_inner_size(PhysicalSize::new(1000, 400));

		use winit::platform::web::WindowExtWebSys;
		web_sys::window()
			.and_then(|win| win.document())
			.and_then(|doc| {
				let dst = doc.get_element_by_id("wasm-example")?;
				let canvas = web_sys::Element::from(window.canvas()?);
				dst.append_child(&canvas).ok()?;
				Some(())
			})
			.expect("Couldn't append canvas to document body.");
	}

	let _res = event_loop.run(move |event, control_flow| match event {
		Event::WindowEvent {
			ref event,
			window_id
		} if window_id == window.id() => match event {
			WindowEvent::CloseRequested
			| WindowEvent::KeyboardInput {
				event:
					KeyEvent {
						state: ElementState::Pressed,
						physical_key: PhysicalKey::Code(KeyCode::Escape),
						..
					},
				..
			} => control_flow.exit(),
			_ => {}
		},
		_ => {}
	});
}

struct State<'a> {
	surface: wgpu::Surface<'a>,
	device:  wgpu::Device,
	queue:   wgpu::Queue,
	config:  wgpu::SurfaceConfiguration,
	size:    winit::dpi::PhysicalSize<u32>,
	// The window must be declared after the surface so it gets dropped after it as the surface
	// contains unsafe references to the window's resources.
	window:  &'a Window
}

impl<'a> State<'a> {
	// Creating some of the wgpu types requires async code
	async fn new(window: &'a Window) -> State<'a> {
		let size = window.inner_size();

		// The instance is a handle to our GPU
		// Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			#[cfg(not(target_arch = "wasm32"))]
			backends: wgpu::Backends::PRIMARY,
			#[cfg(taget_arch = "wasm32")]
			backends: wgpu::Backend::GL,
			..Default::default()
		});

		let surface = instance.create_surface(window).unwrap();

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference:       wgpu::PowerPreference::default(),
				compatible_surface:     Some(&surface),
				force_fallback_adapter: false
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					required_features: wgpu::Features::empty(),
					// WebGL doesn't support all of wgpu's features, so if we're building for the
					// web, we'll have to disable some.
					required_limits:   if cfg!(target_arch = "wasm32") {
						wgpu::Limits::downlevel_webgl2_defaults()
					} else {
						wgpu::Limits::default()
					},
					label:             None,
					memory_hints:      Default::default()
				},
				None // Trace path
			)
			.await
			.unwrap();

		todo!()
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		todo!()
	}

	fn input(&mut self, event: &WindowEvent) -> bool {
		todo!()
	}

	fn update(&mut self) {
		todo!()
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		todo!()
	}
}
