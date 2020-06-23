mod interop;

use interop::{RoInitType, ro_initialize, IDesktopWindowXamlSourceNative};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

use bindings::windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager};
use bindings::windows::ui::xaml::controls::*;

use bindings::windows::ui::xaml::markup::XamlReader;
use bindings::windows::ui::xaml::FrameworkElement;
use winrt::AbiTransferable;
use bindings::windows::ui::xaml::media::SolidColorBrush;
use bindings::windows::ui::Colors;
use bindings::windows::ui::xaml::shapes::Ellipse;

use winrt::{ComInterface, Object, Param, TryInto };
use std::ptr;

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;

    let _win_xaml_manager = WindowsXamlManager::initialize_for_current_thread()?;
    let desktop_source = winrt::factory::<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory>()?.create_instance(Object::default(), &mut Object::default())?;
    let interop: IDesktopWindowXamlSourceNative = desktop_source.clone().into();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("XAML Islands from XML");
    let win32_window_id = window.id();

    let hwnd = window.hwnd();
    interop.attach_to_window(hwnd)?;
    let hwnd_xaml_island = interop.get_window_handle()?;

    let size = window.inner_size();
    unsafe { SetWindowPos(hwnd_xaml_island, ptr::null_mut(), 0, 0, size.width as i32, size.height as i32, /*SWP_SHOWWINDOW*/ 0x40); }

    let stack_panel_root = winrt::factory::<StackPanel, IStackPanelFactory>()?.create_instance(Object::default(), &mut Object::default())?;

    /*
    Sample code from: https://docs.microsoft.com/en-us/uwp/api/windows.ui.xaml.markup.xamlreader.load?view=winrt-19041
    
    string xaml = "<Ellipse Name=\"EllipseAdded\" Width=\"300.5\" Height=\"200\" 
    Fill=\"Red\" xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\"/>";
    object ellipse = XamlReader.Load(xaml);
    //stackPanelRoot is the visual root of a Page in existing XAML markup already loaded by the appmodel
    stackPanelRoot.Children.Add(ellipse as UIElement);
    //walk the tree using XLinq result and cast back to a XAML type to set a property on it at runtime
    var result = (from item in stackPanelRoot.Children
    where (item is FrameworkElement)
    && ((FrameworkElement) item).Name == "EllipseAdded"
    select item as FrameworkElement).FirstOrDefault();
    ((Ellipse) result).Fill = new SolidColorBrush(Colors.Yellow);
     */

    let xaml = "<Ellipse Name=\"EllipseAdded\" Width=\"300.5\" Height=\"200\"
Fill=\"Red\" xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\"/>";

    let ellipse = XamlReader::load(xaml)?;

    stack_panel_root.children()?.append(Param::Borrowed(&ellipse.query()))?;

    for item in stack_panel_root.children()? {
        println!("{:?}", item);
        match item.get_abi() {
            Some(_) => {
                let fe: FrameworkElement = item.try_into()?;
                
                if fe.name()? == "EllipseAdded" {
                    let yellow_brush = SolidColorBrush::new()?;
                    yellow_brush.set_color(Colors::yellow()?)?;
                    let ellipse: Ellipse = item.try_into()?;
                    ellipse.set_fill(yellow_brush)?;
                }
            },
            None => break,
        };
    }

    stack_panel_root.update_layout()?;
    desktop_source.set_content(stack_panel_root)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == win32_window_id => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() {

    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}

#[link(name = "user32")]
extern "stdcall" {
    fn SetWindowPos(
        hwnd: *mut core::ffi::c_void,
        hwnd_insert_after: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32
    ) -> i32;
}
