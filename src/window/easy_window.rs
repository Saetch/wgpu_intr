use std::error::Error;

use winit::{
    event::*, event_loop::{ControlFlow, EventLoop}, keyboard::{Key, KeyCode, NamedKey, PhysicalKey, SmolStr}, window::WindowBuilder
};
use winit::event::WindowEvent::KeyboardInput;

pub fn run() -> Result<(), Box<dyn Error>>{
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;
    
    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::KeyboardInput { event: KeyEvent{
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::KeyF),
                ..
            },
            .. } => {
                println!("You pressed the 'f' key");
            },
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key: Key::Named(NamedKey::Escape),
                    state: ElementState::Pressed,
                    ..
                },

                ..
            }=> control_flow.exit(),

            _ => {}
        },
        _ => {}
    })?;
    Ok(())
}