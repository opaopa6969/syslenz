# fd_usage_pct

What is this?
The percentage of the system-wide file handle limit that is currently in active use.

Why it matters
This is the most actionable file-nr number. It tells you when the host is approaching EMFILE risk.

How to read
- Under 50%: usually fine.
- Around 80%: start looking for leaks.
- Near the limit: open() and socket() failures become likely.

Next check
Compare with fd_allocated and process_count.