#[cfg(any(target_os = "ios", target_os = "android"))]
use gles31_sys::*;

use gl_wrapper::{Buffer, BufferConfig, GLC};
use gm::{Point, Rect, Size};

const RECT: Rect = Rect {
    origin: Point { x: -1.0, y: -1.0 },
    size: Size {
        width: 2.0,
        height: 2.0,
    },
};

const RECT_INDICES: &[u16; 4] = &[0, 1, 3, 2];
const INDICES: &[u16; 4] = &[0, 1, 2, 3];

const FULLSCREEN_VERT: &[f32; 8] = &[
    RECT.origin.x,
    RECT.origin.y,
    RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    RECT.size.width + RECT.origin.x,
    RECT.origin.y,
];

const IMAGE_VERTICES: &[f32; 16] = &[
    RECT.origin.x,
    RECT.origin.y,
    0.0,
    1.0, //|- |
    RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    0.0,
    0.0, //|_ |
    RECT.size.width + RECT.origin.x,
    RECT.size.height + RECT.origin.y,
    1.0,
    0.0, //| _|
    RECT.size.width + RECT.origin.x,
    RECT.origin.y,
    1.0,
    1.0, //| -|
];

pub struct Buffers {
    pub fullscreen: Buffer,
    pub fullscreen_image: Buffer,
    pub fullscreen_outline: Buffer,
}

impl Buffers {
    pub fn init() -> Buffers {
        let fullscreen = Buffer::make(
            &BufferConfig::_2,
            FULLSCREEN_VERT.into(),
            Some(RECT_INDICES.into()),
            GLC!(TRIANGLE_STRIP),
        );

        let fullscreen_image = Buffer::make(
            &BufferConfig::_2_2,
            IMAGE_VERTICES.into(),
            Some(RECT_INDICES.into()),
            GLC!(TRIANGLE_STRIP),
        );

        let fullscreen_outline = Buffer::make(
            &BufferConfig::_2,
            FULLSCREEN_VERT.into(),
            Some(INDICES.into()),
            GLC!(LINE_LOOP),
        );

        Buffers {
            fullscreen,
            fullscreen_image,
            fullscreen_outline,
        }
    }
}