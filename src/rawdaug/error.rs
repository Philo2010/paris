use ash::vk;

#[derive(Debug)]
pub enum RDError {
    // Vulkan specific
    VulkanError(vk::Result),
    // signals the caller to fall back
    VulkanMissedRequrement(&'static str),
    // etc
}

impl From<vk::Result> for RDError {
    fn from(e: vk::Result) -> Self {
        RDError::VulkanError(e)
    }
}