"""Configure LLVM Bazel overlays from a 'raw' imported llvm repository"""

load("@llvm-raw//utils/bazel:configure.bzl", "llvm_configure")

# The subset of LLVM backend targets that should be compiled
_LLVM_TARGETS = [
    # CPUs
    "X86",
    "AArch64",
    "RISCV",
    # NVIDIA GPUs
    "NVPTX",
    # AMD GPUs
    "AMDGPU",    
]

def setup_llvm(name):
    """Build @llvm-project from @llvm-raw using the upstream bazel overlays."""
    llvm_configure(
        name = name,
        targets = _LLVM_TARGETS,
    )