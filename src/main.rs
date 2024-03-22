use crate::args::Args;
use crate::args::Commands;
use clap::Parser;
use donitsi::Donitsi;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod parser;
mod args;
mod commands;
mod window;
mod types;
mod donitsi;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Args = Args::parse();

    // let mut donitsi = Donitsi::new();
    // donitsi.create_window("Donitsi").await;
    // donitsi.run();

    let event_loop = EventLoop::new();
    let window_handle = WindowBuilder::new().build(&event_loop).unwrap();
    let size = window_handle.inner_size();

    event_loop.run(move |event, _, control_flow| {

    })
}
