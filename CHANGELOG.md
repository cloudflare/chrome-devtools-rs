# Changelog

## 🌈 0.0.0-alpha.1

### 🔌 Features

- **Unescape escape sequences in Strings - [jspspike], [pull/40]**

  The Chrome Devtools Protocol will include escape sequences in the messages
  containing strings. In order to display these properly, we now unescape those
  sequences, resulting in much better looking output in the terminal.

  [jspspike]: https://github.com/jspspike [pull/40]:
  https://github.com/EverlastingBugstopper/chrome-devtools-rs/pull/40

### 👏 Fixes

- **Adds accessor type to `RemoteObject` - [jspspike], [pull/38]**

  The `RemoteObject` type did not previously have the `accessor` type, which
  meant certain console calls would fail to deserialize. Now `RemoteObject` can
  parse messages with `accessor`!

  [jspspike]: https://github.com/jspspike [pull/38]:
  https://github.com/EverlastingBugstopper/chrome-devtools-rs/pull/38

- **Don't color strings if color feature is not enabled -
  [EverlastingBugstopper], [pull/36]**

  You can build this crate with the `color` feature and it will display
  console.log calls with color. Before, it would do this even if you didn't pass
  the `color` feature, now the feature flag works as intended.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper [pull/36]:
  https://github.com/EverlastingBugstopper/chrome-devtools-rs/pull/36

### 🔨 Maintenance

- **Updated dependencies - [EverlastingBugstopper], [pull/41]**

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper [pull/41]:
  https://github.com/EverlastingBugstopper/chrome-devtools-rs/pull/41

## 💩 0.0.0-alpha.0

- **First release - [EverlastingBugstopper], [issue/Issue #][pull/pr #]**

  First release of this crate only contains functionality for deserializing and
  displaying calls to console.log.
