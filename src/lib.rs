//! # FerruX Projection Matrix
//! Tool to ease the generation of perspective projection matrices to convert 3D vectors into their 2D projection.
//! 
//! ## Building the matrix
//! To build the matrix just create a new Builder and set the properties that will define the projection matrix:
//! * `near`: Position of the near clip in the z-axis. By default `0.0`.
//! * `far`: Position of the far clip in the z-axis. By default `1000.0`.
//! * `fov`: Field of view in degrees. By default `90.0`.
//! * `width`: Frame width. By default `1280`.
//! * `height`: Frame height. By default `720`.
//! 
//! ```
//! let matrix = ferrux_projection_matrix::ProjectionMatrixBuilder::new()
//!                .set_width(1920)
//!                .set_height(1080)
//!                .set_fov(100.0)
//!                .set_far(2000.0)
//!                .set_near(1.0)
//!                .build();
//! println!("{matrix:?}");
//! ```

use std::f32::consts::PI;

const DEFAULT_NEAR: f32 = 0.0;
const DEFAULT_FAR: f32 = 1000.0;
const DEFAULT_FIELD_OF_VIEW: f32 = 90.0;
const DEFAULT_WIDTH: usize = 1280;
const DEFAULT_HEIGHT: usize = 720;

type Matrix = [[f32; 4]; 4];

/// Builder to construct projection matrices
pub struct ProjectionMatrixBuilder {
    near: f32,
    far: f32,
    fov: f32,
    width: usize,
    height: usize,
}

impl ProjectionMatrixBuilder {
    /// Returns an instance of a builder
    ///
    /// # Default
    /// By default, the matrix builder sets the following defaults:
    /// * `screen_position`: 0.0
    /// * `view_limit`: 1000.0
    /// * `fov`: 90.0
    /// * `width`: 1280
    /// * `height`: 720
    ///
    pub fn new() -> Self {
        Self {
            near: DEFAULT_NEAR,
            far: DEFAULT_FAR,
            fov: DEFAULT_FIELD_OF_VIEW,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }

    /// Sets the near clip position in the z axis
    pub fn set_near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }

    /// Sets the far clip position in the z axis
    pub fn set_far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }

    /// Sets the field of view in grades
    ///
    /// # Panic
    /// If the entered fov is not within the (0, 360) range
    pub fn set_fov(mut self, fov: f32) -> Self {
        if !(0.0..360.0).contains(&fov) {
            panic!("The field of view must be a positive value between 0.0 and 360.0");
        }
        self.fov = fov;
        self
    }

    /// Sets the width of the screen
    pub fn set_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the screen
    pub fn set_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Builds the projection matrix derived from the entered parameters and consumes the builder
    ///
    /// # Panic
    /// If the specified view limit position is less than the specified screen position. The view limit must be
    /// further in the Z-axis.
    pub fn build(&self) -> Matrix {
        let mut matrix = [[0.0; 4]; 4];
        let aspect_ratio = self.width as f32 / self.height as f32;
        let fov_rad: f32 = 1.0 / (self.fov * 0.5 / 180.0 * PI).tan();

        if self.far < self.near {
            panic!("The view limit must be bigger than the screen position, the Z-axis direction is away from the screen");
        }
        let distance = self.far - self.near;

        matrix[0][0] = aspect_ratio * fov_rad;
        matrix[1][1] = fov_rad;
        matrix[2][2] = self.far * distance;
        matrix[3][2] = (-self.far * self.near) / distance;
        matrix[2][3] = 1.0;

        matrix
    }
}

impl Default for ProjectionMatrixBuilder {
    fn default() -> Self {
        Self::new()
	}
}

#[test]
#[should_panic]
fn invalid_fov_low() {
    ProjectionMatrixBuilder::new().set_fov(-1.0);
}

#[test]
#[should_panic]
fn invalid_fov_high() {
    ProjectionMatrixBuilder::new().set_fov(360.0);
}

#[test]
#[should_panic]
fn invalid_zoom() {
    ProjectionMatrixBuilder::new()
        .set_far(0.0)
        .set_near(1.0)
        .build();
}

#[test]
fn default_building() {
    let matrix = ProjectionMatrixBuilder::new().build();
    assert!((DEFAULT_WIDTH as f32 / DEFAULT_HEIGHT as f32 - matrix[0][0]).abs() < 0.0001);
    assert!((1.0 - matrix[1][1]).abs() < 0.0001);
    assert!((DEFAULT_FAR.powf(2.0) - matrix[2][2]).abs() < 0.0001);
    assert!((0.0 - matrix[3][2]).abs() < 0.0001);
    assert!((1.0 - matrix[2][3]).abs() < 0.0001);
}

#[test]
fn custom_building() {
    let width = 1920;
    let height = 1080;
	let aspect_ratio = width as f32 / height as f32;
	let fov = 100.0;
	let fov_rad = 1.0 / (fov * 0.5 / 180.0 * PI).tan();
	let far = 500.0;
	let near = 5.0;
	let zoom = far - near;

    let matrix = ProjectionMatrixBuilder::new()
        .set_width(width)
        .set_height(height)
		.set_fov(fov)
		.set_far(far)
		.set_near(near)
        .build();

    assert!((aspect_ratio * fov_rad - matrix[0][0]).abs() < 0.0001);
    assert!((fov_rad - matrix[1][1]).abs() < 0.0001);
    assert!((far * zoom - matrix[2][2]).abs() < 0.0001);
    assert!(((-far * near)/zoom - matrix[3][2]).abs() < 0.0001);
    assert!((1.0 - matrix[2][3]).abs() < 0.0001);
}
