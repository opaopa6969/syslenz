# Driver, Kernel, and OS Boundary

What is this?
The seam between user space, the kernel, device drivers, and the hardware device itself.

Why it matters
Many incidents are caused by crossing the wrong layer: an app issue that looks like IO, or a driver issue that looks like CPU load.

How to use
- User space: threads, sockets, files, allocators
- Kernel: sched, VM, VFS, networking stack, block layer
- Driver: device programming, interrupts, DMA, queues
- Hardware: controller, media, bus, firmware, thermal limits

Common mistakes
- Blaming the app for a device timeout
- Blaming the disk for a filesystem or scheduler issue
- Forgetting that interrupts and DMA shift work into kernel space

Diagnostic flow
1. Check whether the symptom appears in one process or system-wide.
2. Check whether kernel pressure or interrupts rise with the symptom.
3. Check whether the device itself shows queueing or error signals.
