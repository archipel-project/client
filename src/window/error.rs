use winit::window::BadIcon;

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Failed to read IO: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to read window icon: {0}")]
    BadIcon(#[from] BadIcon),

    #[error("Failed to read image: {0}")]
    ImageError(#[from] image::ImageError)
}

