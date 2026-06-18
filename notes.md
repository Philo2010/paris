# Step 1
 - Intnace (giving the pc a blowjob)
 - get all of the P-devices
# Step 2
 - Pick all of our reqements + queues that P-device -> L-device
 - Figure out how to make a command
 - send the commands to the QUEUE!!!!!!
# Step 3
 - use winit to create a window surface
 - Connect to the surface KHR
 - Create a swapchain
# Step 4
 - wrap our images in a ImageView or Framebuffer
 - ImageView
    - The part of the image we are handling
 - FrameBuffer
    - A refance of many imageviews
# Step 5
 - OLD: RenderPass + Framebuffer to define subpasses
 - NEW: what im going tp learn ins dyanmic rendering
# Step 6
 - Create our VkPipeline objects
   - defines stuff like viewport size etc
 - create and define vkShaderModule objects
# Step 7
 - Create a CommandBuffer from a CommandPool
 - Use dytanmic rendering with BeginRendering to create object
# Step 8
 - Get an image from the swap chain
 - find the approate command buffer
 - submit it to a graphics queue
 - return the image to the swap chain and present


# How object creation works
```
vk::XXXCreateInfo createInfo{};
createInfo.sType = vk::StructureType::eXXXCreateInfo;
createInfo.pNext = nullptr;
createInfo.foo = ...;
createInfo.bar = ...;

vk::XXX object;


try {
    object = device.createXXX(createInfo);
} catch (vk::SystemError& err) {
    std::cerr << "Failed to create object: " << err.what() << std::endl;
    return false;
}
```