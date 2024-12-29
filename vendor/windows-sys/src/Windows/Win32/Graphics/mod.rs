#[cfg(feature = "Win32_Graphics_Dwm")]
pub mod Dwm;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub mod Gdi;
#[cfg(feature = "Win32_Graphics_GdiPlus")]
pub mod GdiPlus;
#[cfg(feature = "Win32_Graphics_Hlsl")]
pub mod Hlsl;
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub mod OpenGL;
#[cfg(feature = "Win32_Graphics_Printing")]
pub mod Printing;
