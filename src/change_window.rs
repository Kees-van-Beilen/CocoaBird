//this is a copy of the change_window function in bevy_winit
use bevy::prelude::{NonSendMut,ResMut,Windows,EventWriter};
use bevy::winit::{WinitWindows,get_best_videomode,get_fitting_videomode};
use bevy::window::{WindowClosed,WindowScaleFactorChanged};
use bevy::math::{ivec2, DVec2, UVec2, Vec2};
// use winit::{
//     dpi::{LogicalSize, PhysicalPosition},
//     event::{self, DeviceEvent, Event, StartCause, WindowEvent},
//     event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
// };
use crate::CocoaBirdWindows;
use super::converters;
use bevy::log::{error,warn};

pub fn change_window(
    mut winit_windows: NonSendMut<CocoaBirdWindows>,
    mut windows: ResMut<Windows>,
    mut window_dpi_changed_events: EventWriter<WindowScaleFactorChanged>,
    mut window_close_events: EventWriter<WindowClosed>,
) {
    // winit::window::Fullscreen
    let mut removed_windows = vec![];
    for bevy_window in windows.iter_mut() {
        let id = bevy_window.id();
        for command in bevy_window.drain_commands() {
            match command {
                bevy::window::WindowCommand::SetWindowMode {
                    mode,
                    resolution:
                        UVec2 {
                            x: width,
                            y: height,
                        },
                } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // match mode {
                    //     bevy::window::WindowMode::BorderlessFullscreen => {
                    //         // winit::window::Fullscreen::Borderless(None);
                    //         // bevy::window::
                    //         window
                    //             .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    //     }
                    //     bevy::window::WindowMode::Fullscreen => {
                    //         window.set_fullscreen(Some(winit::window::Fullscreen::Exclusive(
                    //             get_best_videomode(&window.current_monitor().unwrap()),
                    //         )));
                    //     }
                    //     bevy::window::WindowMode::SizedFullscreen => window.set_fullscreen(Some(
                    //         winit::window::Fullscreen::Exclusive(get_fitting_videomode(
                    //             &window.current_monitor().unwrap(),
                    //             width,
                    //             height,
                    //         )),
                    //     )),
                    //     bevy::window::WindowMode::Windowed => window.set_fullscreen(None),
                    // }
                }
                bevy::window::WindowCommand::SetTitle { title } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_title(&title);
                }
                bevy::window::WindowCommand::SetScaleFactor { scale_factor } => {
                    // window_dpi_changed_events.send(WindowScaleFactorChanged { id, scale_factor });
                }
                bevy::window::WindowCommand::SetResolution {
                    logical_resolution:
                        Vec2 {
                            x: width,
                            y: height,
                        },
                    scale_factor,
                } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_inner_size(
                    //     winit::dpi::LogicalSize::new(width, height)
                    //         .to_physical::<f64>(scale_factor),
                    // );
                }
                bevy::window::WindowCommand::SetPresentMode { .. } => (),
                bevy::window::WindowCommand::SetResizable { resizable } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_resizable(resizable);
                }
                bevy::window::WindowCommand::SetDecorations { decorations } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_decorations(decorations);
                }
                bevy::window::WindowCommand::SetCursorIcon { icon } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_cursor_icon(converters::convert_cursor_icon(icon));
                }
                bevy::window::WindowCommand::SetCursorLockMode { locked } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window
                    //     .set_cursor_grab(locked)
                    //     .unwrap_or_else(|e| error!("Unable to un/grab cursor: {}", e));
                }
                bevy::window::WindowCommand::SetCursorVisibility { visible } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_cursor_visible(visible);
                }
                bevy::window::WindowCommand::SetCursorPosition { position } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // let inner_size = window.inner_size().to_logical::<f32>(window.scale_factor());
                    // window
                    //     .set_cursor_position(winit::dpi::LogicalPosition::new(
                    //         position.x,
                    //         inner_size.height - position.y,
                    //     ))
                    //     .unwrap_or_else(|e| error!("Unable to set cursor position: {}", e));
                }
                bevy::window::WindowCommand::SetMaximized { maximized } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_maximized(maximized);
                }
                bevy::window::WindowCommand::SetMinimized { minimized } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_minimized(minimized);
                }
                bevy::window::WindowCommand::SetPosition { position } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // window.set_outer_position(PhysicalPosition {
                    //     x: position[0],
                    //     y: position[1],
                    // });
                }
                bevy::window::WindowCommand::Center(monitor_selection) => {
                    // let window = winit_windows.get_window(id).unwrap();

                    // use bevy::window::MonitorSelection::*;
                    // let maybe_monitor = match monitor_selection {
                    //     Current => window.current_monitor(),
                    //     Primary => window.primary_monitor(),
                    //     Number(n) => window.available_monitors().nth(n),
                    // };

                    // if let Some(monitor) = maybe_monitor {
                    //     let screen_size = monitor.size();

                    //     let window_size = window.outer_size();

                    //     window.set_outer_position(PhysicalPosition {
                    //         x: screen_size.width.saturating_sub(window_size.width) as f64 / 2.
                    //             + monitor.position().x as f64,
                    //         y: screen_size.height.saturating_sub(window_size.height) as f64 / 2.
                    //             + monitor.position().y as f64,
                    //     });
                    // } else {
                    //     warn!("Couldn't get monitor selected with: {monitor_selection:?}");
                    // }
                }
                bevy::window::WindowCommand::SetResizeConstraints { resize_constraints } => {
                    // let window = winit_windows.get_window(id).unwrap();
                    // let constraints = resize_constraints.check_constraints();
                    // let min_inner_size = LogicalSize {
                    //     width: constraints.min_width,
                    //     height: constraints.min_height,
                    // };
                    // let max_inner_size = LogicalSize {
                    //     width: constraints.max_width,
                    //     height: constraints.max_height,
                    // };

                    // window.set_min_inner_size(Some(min_inner_size));
                    // if constraints.max_width.is_finite() && constraints.max_height.is_finite() {
                    //     window.set_max_inner_size(Some(max_inner_size));
                    // }
                }
                bevy::window::WindowCommand::Close => {
                    // Since we have borrowed `windows` to iterate through them, we can't remove the window from it.
                    // Add the removal requests to a queue to solve this
                    removed_windows.push(id);
                    // No need to run any further commands - this drops the rest of the commands, although the `bevy_window::Window` will be dropped later anyway
                    break;
                }
            }
        }
    }
    if !removed_windows.is_empty() {
        for id in removed_windows {
            // Close the OS window. (The `Drop` impl actually closes the window)
            let _ = winit_windows.remove_window(id);
            // Clean up our own data structures
            windows.remove(id);
            window_close_events.send(WindowClosed { id });
        }
    }
}