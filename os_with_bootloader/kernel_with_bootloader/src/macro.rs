#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        unsafe {
            if let Some(ref mut writer) = crate::FRAME_BUFFER_WRITER {
                let _ = write!(writer, $($arg)*);
            }
   }
}};
}