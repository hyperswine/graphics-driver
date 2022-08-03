// DRM entry point

// wraps around multiple GPUs. Handles requests for multiple gpus

pub struct DirectRenderManager {}

// -----------------
// Spectral RayTrace
// -----------------

// 1. convert a RayTrace scene description (.yml) through compiler into .rt binary
// 2. convert a cpu game program into ELF. When the functions (these functions below) are called, copy command buffer and tell RTU to ray trace

fn cast_ray() {}

// Prob better on userland!!
