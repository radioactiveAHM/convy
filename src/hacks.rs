#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct GaussianBlurParametersX {
	x_axis_kernel_size: u32,
	x_axis_sigma: f32,
	y_axis_kernel_size: u32,
	y_axis_sigma: f32,
}

impl GaussianBlurParametersX {
	pub fn new(kernel_size: u32, sigma: f32) -> Self {
		Self {
			x_axis_kernel_size: kernel_size,
			x_axis_sigma: sigma,
			y_axis_kernel_size: kernel_size,
			y_axis_sigma: sigma,
		}
	}
	pub fn transmute(self) -> image::imageops::GaussianBlurParameters {
		unsafe { std::mem::transmute::<GaussianBlurParametersX, image::imageops::GaussianBlurParameters>(self) }
	}
}
