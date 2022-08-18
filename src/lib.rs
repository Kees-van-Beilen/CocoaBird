use std::fs::File;
use std::os::raw::c_void;
use std::process::exit;

use bevy::app::PluginGroupBuilder;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool};
use display_link::macos::DisplayLink;
use cocoa::appkit::*;
use bevy::winit::{WinitSettings};
use bevy::window::{
    CreateWindow, CursorEntered, CursorLeft, CursorMoved, FileDragAndDrop, ModifiesWindows,
    ReceivedCharacter, RequestRedraw, WindowBackendScaleFactorChanged, WindowCloseRequested,
    WindowClosed, WindowCreated, WindowFocused, WindowMoved, WindowResized,
    WindowScaleFactorChanged, Windows,WindowId
};
mod change_window;
mod converters;


pub struct CocoaBirdDefaultPlugins;
pub struct CocoaBirdPlugin;
#[derive(Default)]
struct WinitCreateWindowReader(ManualEventReader<CreateWindow>);


impl PluginGroup for CocoaBirdDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::time::TimePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::hierarchy::HierarchyPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        // group.add(bevy_input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(CocoaBirdPlugin);
        group.add(bevy::asset::AssetPlugin::default());
        // #[cfg(feature = "debug_asset_server")]
        // group.add(bevy_asset::debug_asset_server::DebugAssetServerPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());
        // group.add(bevy_winit::WinitPlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::core_pipeline::CorePipelinePlugin::default());
        group.add(bevy::sprite::SpritePlugin::default());
        group.add(bevy::text::TextPlugin::default());
        // group.add(bevy::ui::UiPlugin::default());
        group.add(bevy::pbr::PbrPlugin::default());
        group.add(bevy::gltf::GltfPlugin::default());
        group.add(bevy::audio::AudioPlugin::default());
        // group.add(bevy::gilrs::GilrsPlugin::default());
        group.add(bevy::animation::AnimationPlugin::default());
    }
}

impl Plugin for CocoaBirdPlugin{
    fn build(&self, app: &mut App) {
        //register this proccess as aan nsapp
        unsafe{
            //this only crashes under rare conditions and the error will be visible in the system's console
            let nsapp = NSApp();
            nsapp.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            nsapp.activateIgnoringOtherApps_(true);
        }
        app.init_non_send_resource::<CocoaBirdWindows>()
            .init_resource::<WinitSettings>()
            .set_runner(cocoa_bird_runner)
            .add_system_to_stage(CoreStage::PostUpdate, change_window::change_window.label(ModifiesWindows));

        //we are not going to use the winit event loop
        // let event_loop = EventLoop::new();
        // event_loop.run(event_handler)

        let mut create_window_reader = WinitCreateWindowReader::default();

        handle_create_window_events(&mut app.world, &mut create_window_reader.0);
        app.insert_resource(create_window_reader);
    }
}

static mut APP_REF: *mut App = std::ptr::null_mut();
static mut FRAME_COUNT: u32 = 0;
// static mut GUARD: *mut ProfilerGuard = std::ptr::null_mut();
unsafe fn run_loop(){

    // println!("update");
    (*APP_REF).update();
    // FRAME_COUNT+=1;
    // if FRAME_COUNT == 5*60 {
    //     // flame::end("game");
    //     // flame::end("spawn_sync");/
        
    //     // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
        
    //     NSApplication::stop_(NSApp(),cocoa::base::nil);
    // }
}

fn cocoa_bird_runner(app:App){
    println!("running with cocoa bird");
    unsafe{
        // GUARD = Box::into_raw(Box::new(guard));
        let nsapp = NSApp();
        nsapp.activateIgnoringOtherApps_(cocoa::base::YES);

        let wrapper = Box::new(app);
        APP_REF = Box::into_raw(wrapper);

        let displayLink = DisplayLink::new( move |p|{
            fruity::dispatch::DispatchQueue::spawn_sync(fruity::dispatch::DispatchQueue::main(), ||{run_loop()});
        });
        if displayLink.is_none() {
            panic!("Failed to init display-link");
        }
        let mut u = displayLink.unwrap();
        u.set_current_display(core_graphics::display::CGMainDisplayID());
        if u.resume().is_err() {
            panic!("Failed to start display-link");
        }
        nsapp.run();
        println!("quiting");
        exit(0);
    }

}




fn handle_create_window_events(
    world: &mut World,
    create_window_event_reader: &mut ManualEventReader<CreateWindow>,
) {
    let world = world.cell();
    let mut winit_windows = world.non_send_resource_mut::<CocoaBirdWindows>();
    let mut windows = world.resource_mut::<Windows>();
    let create_window_events = world.resource::<Events<CreateWindow>>();
    let mut window_created_events = world.resource_mut::<Events<WindowCreated>>();
    let mut window_resized_events = world.resource_mut::<Events<WindowResized>>();
    for create_window_event in create_window_event_reader.iter(&create_window_events) {

        let window = winit_windows.create_window(
            create_window_event.id,
            &create_window_event.descriptor,
        );

        window_resized_events.send(WindowResized {
            id: create_window_event.id,
            width: window.width(),
            height: window.height(),
        });
        windows.add(window);
        window_created_events.send(WindowCreated {
            id: create_window_event.id,
        });

    }
}

#[derive(Debug, Default)]
pub struct CocoaBirdWindows {
    pub windows: HashMap<WindowId, cocoa::base::id>,
    pub window_id_to_winit: HashMap<WindowId, WindowId>,
    pub winit_to_window_id: HashMap<WindowId, WindowId>,
    // Some winit functions, such as `set_window_icon` can only be used from the main thread. If
    // they are used in another thread, the app will hang. This marker ensures `WinitWindows` is
    // only ever accessed with bevy's non-send functions and in NonSend systems.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

fn NSMakeRect(x:f64,y:f64,width:f64,height:f64)->NSRect{
    return NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
}

impl CocoaBirdWindows {
    pub fn create_window(
        &mut self,
        window_id: WindowId,
        window_descriptor: &WindowDescriptor,
    ) -> Window {
        use cocoa::base::{nil,NO};
        unsafe{
            let nswindow = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(NSMakeRect(0.0,0.0,window_descriptor.width.into(),window_descriptor.height.into()), NSWindowStyleMask::NSTitledWindowMask, NSBackingStoreBuffered, NO).autorelease();
            // if(WindowPosition::)
            NSWindow::center(nswindow);
            NSWindow::makeKeyAndOrderFront_(nswindow, nil);
            let mut handle = raw_window_handle::AppKitHandle::empty();
            handle.ns_window = nswindow as *mut c_void;
            handle.ns_view = NSWindow::contentView(nswindow) as *mut c_void;

            return Window::new(
                window_id,
                window_descriptor,
                (window_descriptor.width*2.0) as u32,
                (window_descriptor.height*2.0) as u32,
                1.0,
                None,
                raw_window_handle::RawWindowHandle::AppKit(handle),
            )
        }
    }

    pub fn get_window(&self, id: WindowId) -> Option<&cocoa::base::id> {
        self.windows.get(&id)
    }

    pub fn get_window_id(&self, id: WindowId) -> Option<WindowId> {
        self.winit_to_window_id.get(&id).cloned()
    }

    pub fn remove_window(&mut self, id: WindowId) -> Option<cocoa::base::id> {
        let winit_id = self.window_id_to_winit.remove(&id)?;
        // Don't remove from winit_to_window_id, to track that we used to know about this winit window
        self.windows.remove(&winit_id)
    }
}
