#[cfg(any(target_os = "ios", target_os = "android"))]
use gles31_sys::*;
use gm::{
    flat::{Point, Size},
    Color,
};

#[derive(Debug)]
pub struct Shader {
    pub name: String,

    program:    u32,
    color:      i32,
    size:       i32,
    selected:   i32,
    resolution: i32,
    position:   i32,
    rotation:   i32,
    flipped:    i32,
    flipped_y:  i32,
    scale:      i32,

    camera_rotation: i32,
    camera_position: i32,
}

fn get_uniform(program: u32, lit: &str) -> i32 {
    use std::ffi::CString;
    let c_str = CString::new(lit).unwrap();
    GL!(GetUniformLocation, program, c_str.as_ptr())
}

impl Shader {
    pub fn new(program: u32, name: String) -> Shader {
        Shader {
            name,
            program,
            color: get_uniform(program, "color"),
            size: get_uniform(program, "size"),
            selected: get_uniform(program, "selected"),
            resolution: get_uniform(program, "resolution"),
            position: get_uniform(program, "position"),
            rotation: get_uniform(program, "rotation"),
            flipped: get_uniform(program, "flipped"),
            flipped_y: get_uniform(program, "flipped_y"),
            scale: get_uniform(program, "scale"),
            camera_rotation: get_uniform(program, "camera_rotation"),
            camera_position: get_uniform(program, "camera_position"),
        }
    }

    pub fn enable(&self) {
        GL!(UseProgram, self.program)
    }

    pub fn set_color(&self, color: Color) {
        debug_assert!(self.color >= 0, "Invalid shader uniform");
        GL!(Uniform4fv, self.color, 1, &color.r)
    }

    pub fn set_size(&self, size: Size) {
        debug_assert!(self.size >= 0, "Invalid shader uniform");
        GL!(Uniform2fv, self.size, 1, &size.width)
    }

    pub fn set_selected(&self, selected: bool) {
        debug_assert!(self.selected >= 0, "Invalid shader uniform");
        GL!(Uniform1i, self.selected, selected.into())
    }

    pub fn set_resolution(&self, resolution: Size) {
        debug_assert!(self.resolution >= 0, "Invalid shader uniform");
        GL!(Uniform2fv, self.resolution, 1, &resolution.width)
    }

    pub fn set_position(&self, point: Point) {
        debug_assert!(self.position >= 0, "Invalid shader uniform");
        GL!(Uniform2fv, self.position, 1, &point.x)
    }

    pub fn set_rotation(&self, angle: f32) {
        debug_assert!(self.rotation >= 0, "Invalid shader uniform");
        GL!(Uniform1f, self.rotation, angle)
    }

    pub fn set_camera_rotation(&self, angle: f32) {
        debug_assert!(self.camera_position >= 0, "Invalid shader uniform");
        GL!(Uniform1f, self.camera_rotation, angle)
    }

    pub fn set_camera_position(&self, pos: Point) {
        debug_assert!(self.camera_position >= 0, "Invalid shader uniform");
        GL!(Uniform2fv, self.camera_position, 1, &pos.x)
    }

    pub fn set_flipped(&self, flipper: bool) {
        debug_assert!(self.flipped >= 0, "Invalid shader uniform");
        GL!(Uniform1i, self.flipped, flipper.into())
    }

    pub fn set_flipped_y(&self, flipper: bool) {
        debug_assert!(self.flipped_y >= 0, "Invalid shader uniform");
        GL!(Uniform1i, self.flipped_y, flipper.into())
    }

    pub fn set_scale(&self, scale: f32) {
        debug_assert!(self.scale >= 0, "Invalid shader uniform");
        GL!(Uniform1f, self.scale, scale)
    }
}