#![cfg(mobile)]

use std::ffi::{c_float, c_int, c_ulong};

use gl_wrapper::GLWrapper;

use crate::{mobile::MobileKeyEvent, App, TestEngineAction};

static mut APP: Option<Box<dyn App>> = None;

#[no_mangle]
pub extern "C" fn set_screen_size(width: c_int, height: c_int) {
    trace!("set_screen_size");
    unsafe { APP.as_mut().unwrap().core().set_screen_size(width, height) }
}

#[no_mangle]
pub extern "C" fn update_screen() -> TestEngineAction {
    unsafe { APP.as_mut().unwrap().core().update_screen() }
}

#[no_mangle]
pub extern "C" fn on_touch(id: c_ulong, x: c_float, y: c_float, event: c_int) {
    unsafe { APP.as_mut().unwrap().core().on_touch(id as _, x, y, event) }
}

#[no_mangle]
pub extern "C" fn set_gyro(pitch: c_float, roll: c_float, yaw: c_float) {
    unsafe { APP.as_mut().unwrap().core().set_gyro(pitch, roll, yaw) }
}

#[no_mangle]
pub extern "C" fn add_key(char: u8, event: MobileKeyEvent) {
    unsafe { APP.as_mut().unwrap().core().add_key(char, event) }
}

extern "C" {
    fn make_app(
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolution_x: c_int,
        resolution_y: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) -> Box<dyn App>;
}

#[no_mangle]
pub extern "C" fn set_monitor(
    ppi: c_int,
    scale: c_float,
    refresh_rate: c_int,
    resolution_x: c_int,
    resolution_y: c_int,
    width: c_float,
    height: c_float,
    diagonal: c_float,
) {
    unsafe {
        let app = make_app(
            ppi,
            scale,
            refresh_rate,
            resolution_x,
            resolution_y,
            width,
            height,
            diagonal,
        );

        APP = Some(app);
    }
}

#[no_mangle]
pub extern "C" fn opengl_ready() {
    GLWrapper::save_default_framebuffer_id();
}

#[cfg(android)]
#[allow(non_snake_case)]
pub mod android {

    use android_ndk_sys::{jclass, jobject, JNIEnv};

    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_setAssetManager(
        env: JNIEnv,
        _: jclass,
        asset_manager: jobject,
    ) {
        rtools::file::set_asset_manager(env, asset_manager);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setScreenSize(
        _: JNIEnv,
        _: jclass,
        width: c_int,
        height: c_int,
    ) {
        set_screen_size(width as _, height as _);
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_update(_: JNIEnv, _: jclass) {
        update_screen();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MainActivity_onTouch(
        _: JNIEnv,
        _: jclass,
        id: c_ulong,
        x: c_float,
        y: c_float,
        event: c_int,
    ) {
        on_touch(id + 1, x, y, event)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_testengine_MyGLRenderer_setMonitor(
        _: JNIEnv,
        _: jclass,
        ppi: c_int,
        scale: c_float,
        refresh_rate: c_int,
        resolutionX: c_int,
        resolutionY: c_int,
        width: c_float,
        height: c_float,
        diagonal: c_float,
    ) {
        set_monitor(
            ppi,
            scale,
            refresh_rate,
            resolutionX,
            resolutionY,
            width,
            height,
            diagonal,
        )
    }
}
