//! # FerruX Projection Matrix
//! Tool to ease the generation of perspective projection matrices to convert 3D vectors into their 2D projection.

const DEFAULT_SCREEN_POSITION: f32 = 0.0;
const DEFAULT_VIEW_LIMIT: f32 = 1000.0;
const DEFAULT_FIELD_OF_VIEW: f32 = 90.0;
const DEFAULT_WIDTH: usize = 1280;
const DEFAULT_HEIGHT: usize = 720;

type Matrix = [[f32; 4]; 4];

/// Builder to construct projection matrices
pub struct ProjectionMatrixBuilder {
	screen_position: f32,
	view_limit: f32,
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
			screen_position: DEFAULT_SCREEN_POSITION, 
			view_limit: DEFAULT_VIEW_LIMIT,
			fov: DEFAULT_FIELD_OF_VIEW,
			width: DEFAULT_WIDTH,
			height: DEFAULT_HEIGHT,
		}
	}

	/// Sets the spatial screen position in the z axis
	pub fn set_screen_position(mut self, position: f32) -> Self {
		self.screen_position = position;
		self
	}

	/// Sets the view limit of the rendering in the z axis
	pub fn set_view_limit(mut self, limit: f32) -> Self {
		self.view_limit = limit;
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
	pub fn build(&self) -> Matrix {
		let mut matrix = [[0.0; 4]; 4];
		let aspect_ratio = self.width as f32 / self.height as f32;
		let fov_rad: f32 = 1.0 / (self.fov * 0.5 / 180.0 * 3.14159).tan();
		let distance = self.view_limit - self.screen_position;
		
		matrix[0][0] = aspect_ratio * fov_rad;
		matrix[1][1] = fov_rad;
		matrix[2][2] = self.view_limit * distance;
		matrix[3][2] = (-self.view_limit * self.screen_position) / distance;
		matrix[2][3] = 1.0;

		matrix
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
