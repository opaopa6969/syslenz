# syslenz demo recording with vhs (charmbracelet/vhs)
# Install: go install github.com/charmbracelet/vhs@latest
# Run: vhs docs/demo.tape

Output docs/assets/demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set TypingSpeed 50ms

# Build first
Hide
Type "source ~/.cargo/env && cargo build --release 2>/dev/null"
Enter
Sleep 5s
Show

# Start syslenz (Dashboard is default)
Type "target/release/syslenz"
Enter
Sleep 2s

# Show Dashboard
Sleep 2s

# Switch to Classic mode
Type "O"
Sleep 1.5s

# Navigate to meminfo
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

# Drill into meminfo
Enter
Sleep 1.5s

# Scroll through fields
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

# Show help (Normal)
Type "?"
Sleep 1.5s

# Show help (Detailed)
Type "?"
Sleep 1.5s

# Show help (Extra - with diagnostics)
Type "?"
Sleep 2s

# Turn off help
Type "?"
Sleep 500ms

# Back to sidebar
Type "h"
Sleep 500ms

# Navigate to loadavg
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

# Show graph
Enter
Sleep 500ms
Type "g"
Sleep 2s

# Back
Type "h"
Sleep 500ms
Type "h"
Sleep 500ms

# Show diff view
Type "d"
Sleep 2s

# Show Diagnostics
Type "X"
Sleep 2s

# Show Category Guide
Type "C"
Sleep 2s
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Switch language to Japanese
Type "L"
Sleep 2s

# Back to Dashboard
Type "D"
Sleep 2s

# Quit
Type "q"
Sleep 500ms
