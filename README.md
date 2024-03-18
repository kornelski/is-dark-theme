## Check whether the system is set to a Dark Mode style

This checks a *global* system default. It is intended for headless tools or programs that can't access their windows.
It may not be accurate when specific screens or applications have a different theme set.
If you control your GUI, please check window-specific properties instead (on macOS that is `NSAppearance` protocol).

On macOS this crate uses Core Foundation to read a `AppleInterfaceStyle` global setting, which is equivalent of:

```bash
defaults read -g AppleInterfaceStyle
```

On other platforms only `None` is returned. Please submit pull requests for more OSes!
