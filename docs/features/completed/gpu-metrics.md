# GPU Metrics

- **Status**: completed
- **Version**: v1.3.0
- **Source module**: `src/sources/gpu.rs`
- **User docs**: [en](../../en/sources.md#gpu-v130) | [ja](../../ja/sources.md)

## Summary

GPU monitoring support for NVIDIA and AMD GPUs. NVIDIA metrics are collected via the
NVML library (`libnvidia-ml.so`), while AMD metrics are read from sysfs
(`/sys/class/drm/card*/device/`). Metrics include utilization, memory usage,
temperature, power draw, clock speed, fan speed, and per-process GPU usage.

## Key capabilities

- Auto-detection of NVIDIA and AMD GPUs at startup
- Per-GPU metrics: utilization, VRAM used/total, temperature, power, clock, fan speed
- Per-process GPU memory and utilization tracking (`gpu/processes` source)
- Multi-GPU support with per-device indexing
- GPU section in Dashboard view with AA bar graphs
- Metrics exported via OTLP and Prometheus endpoints
- Graceful fallback: GPU sources are silently omitted when no GPU is detected
