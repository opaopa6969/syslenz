# available_disk

What is this?
Bytes available for new writes on the root filesystem.

Why it matters
This is the operational free space number. It is the one that prevents log loss and failed deploys.

How to read
- Keep a healthy buffer, not just a few megabytes.
- Fast downward trends matter more than a single snapshot.
- If it keeps falling, find the writer before the filesystem fills.

Next check
Compare with root_use_pct and file descriptor usage.