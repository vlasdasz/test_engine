use std::{
    default::Default,
    path::{Path, PathBuf},
    ptr,
};

use rtools::data_manager::{DataManager, DataStorage};

use crate::Image;

type ImageStorage = DataStorage<Image>;

static mut PATH: *const PathBuf = ptr::null_mut();
static mut IMAGE_STORAGE: *mut ImageStorage = ptr::null_mut();

impl DataManager<Image> for Image {
    fn path() -> PathBuf {
        unsafe { PATH.as_ref().unwrap().clone() }
    }

    fn set_path(path: &Path) {
        unsafe {
            PATH = Box::into_raw(Box::new(path.to_path_buf()));
        }
    }

    fn storage() -> &'static mut DataStorage<Image> {
        unsafe {
            if IMAGE_STORAGE.is_null() {
                IMAGE_STORAGE = Box::into_raw(Box::new(Default::default()));
            }
            IMAGE_STORAGE.as_mut().unwrap()
        }
    }
}
