# fix-cursor

A little CLI in Rust. 
It manages a list of Flatpak apps and if you "enforce" that list it will make it so
those apps use XWayland. 
That's it

## installation

1. Download the binary from the latest release
2. Extract that:
```
tar -xzf fix-cursor-linux-x86_64.tar.gz
```
3. Install that:
```
sudo install -Dm755 fix-cursor/fix-cursor /usr/local/bin/fix-cursor
```
**To uninstall:**
```
sudo rm /usr/local/bin/fix-cursor
```
