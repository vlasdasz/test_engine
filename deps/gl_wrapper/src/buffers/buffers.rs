#[cfg(mobile)]
use gles31_sys::*;
use gm::flat::Rect;
use rtools::static_get;

use crate::{Buffer, BufferConfig};

const RECT: Rect = (-1, -1, 2, 2).into();

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
    pub full:         Buffer,
    pub full_image:   Buffer,
    pub full_outline: Buffer,
}

static_get!(Buffers);

impl Default for Buffers {
    fn default() -> Buffers {
        trace!("Initializing buffers");

        let full = Buffer::make(
            &BufferConfig::_2,
            FULLSCREEN_VERT.into(),
            Some(RECT_INDICES.into()),
            GLC!(TRIANGLE_STRIP),
        );

        let full_image = Buffer::make(
            &BufferConfig::_2_2,
            IMAGE_VERTICES.into(),
            Some(RECT_INDICES.into()),
            GLC!(TRIANGLE_STRIP),
        );

        let full_outline = Buffer::make(
            &BufferConfig::_2,
            FULLSCREEN_VERT.into(),
            Some(INDICES.into()),
            GLC!(LINE_LOOP),
        );

        trace!("Buffers: OK");

        Buffers {
            full,
            full_image,
            full_outline,
        }
    }
}