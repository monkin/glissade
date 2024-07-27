#[cfg(feature = "cgmath")]
mod cgmath;
#[cfg(feature = "euclid")]
mod euclid;
#[cfg(feature = "glam")]
mod glam;
#[cfg(feature = "nalgebra")]
mod nalgebra;
#[cfg(feature = "palette")]
mod palette;
#[cfg(not(feature = "web-time"))]
mod std_time;
#[cfg(feature = "web-time")]
mod web_time;
