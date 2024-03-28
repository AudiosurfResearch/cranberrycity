use egui_d3d9::EguiDx9;
use retour::static_detour;

use egui::{Context, FontData, FontDefinitions, FontFamily, FontTweak, RichText};
use std::{intrinsics::transmute, sync::Once};
use windows::{
    core::{s, HRESULT, PCSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::{
            Direct3D9::{IDirect3DDevice9, D3DPRESENT_PARAMETERS},
            Gdi::RGNDATA,
        },
        System::Console::AllocConsole,
        UI::WindowsAndMessaging::{
            CallWindowProcW, FindWindowA, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC,
        },
    },
};

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32, _reserved: *mut ()) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main_thread(hinst) });
    }

    1
}

static mut APP: Option<EguiDx9<i32>> = None;
static mut OLD_WND_PROC: Option<WNDPROC> = None;

static_detour! {
    static PresentHook: unsafe extern "stdcall" fn(IDirect3DDevice9, *const RECT, *const RECT, HWND, *const RGNDATA) -> HRESULT;
    static ResetHook: unsafe extern "stdcall" fn(IDirect3DDevice9, *const D3DPRESENT_PARAMETERS) -> HRESULT;
}

type FnPresent = unsafe extern "stdcall" fn(
    IDirect3DDevice9,
    *const RECT,
    *const RECT,
    HWND,
    *const RGNDATA,
) -> HRESULT;
type FnReset =
    unsafe extern "stdcall" fn(IDirect3DDevice9, *const D3DPRESENT_PARAMETERS) -> HRESULT;

fn hk_present(
    dev: IDirect3DDevice9,
    source_rect: *const RECT,
    dest_rect: *const RECT,
    window: HWND,
    rgn_data: *const RGNDATA,
) -> HRESULT {
    unsafe {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let window = FindWindowA(PCSTR(std::ptr::null()), s!("Audiosurf"));

            APP = Some(EguiDx9::init(&dev, window, ui, 0, true));

            OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
                window,
                GWLP_WNDPROC,
                hk_wnd_proc as usize as _,
            )));
        });

        APP.as_mut().unwrap().present(&dev);

        PresentHook.call(dev, source_rect, dest_rect, window, rgn_data)
    }
}

fn hk_reset(
    dev: IDirect3DDevice9,
    presentation_parameters: *const D3DPRESENT_PARAMETERS,
) -> HRESULT {
    unsafe {
        APP.as_mut().unwrap().pre_reset();

        ResetHook.call(dev, presentation_parameters)
    }
}

unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    APP.as_mut().unwrap().wnd_proc(msg, wparam, lparam);

    CallWindowProcW(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}

fn ui(ctx: &Context, _i: &mut i32) {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        let mut fonts = FontDefinitions::default();
        let tweak = FontTweak::default();
        fonts.font_data.insert(
            "inter".to_owned(),
            FontData::from_static(include_bytes!("../res/Inter-Regular.ttf")).tweak(tweak),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "inter".to_owned());
        ctx.set_fonts(fonts);
        // egui_extras::install_image_loaders(ctx);
    });

    egui::containers::Window::new("Cranberry City").show(ctx, |ui| {
        ui.label(RichText::new("at least it's not Quest3DTamperer™").italics());
    });
}

#[allow(unused_must_use)]
unsafe fn main_thread(_hinst: usize) {
    unsafe {
        AllocConsole();
    }

    let methods = shroud::directx9::methods().unwrap();

    let reset = methods.device_vmt()[16];
    let present = methods.device_vmt()[17];

    eprintln!("Present: {:X}", present as usize);
    eprintln!("Reset: {:X}", reset as usize);

    let present: FnPresent = std::mem::transmute(present);
    let reset: FnReset = std::mem::transmute(reset);

    PresentHook
        .initialize(present, hk_present)
        .unwrap()
        .enable()
        .unwrap();

    ResetHook
        .initialize(reset, hk_reset)
        .unwrap()
        .enable()
        .unwrap();
}
