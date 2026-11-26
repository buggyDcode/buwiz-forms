# Android money box after `fix/money-box-no-live-format`

Device: `buwiz` x86_64 emulator. Input: real hold-taps on the numeric keypad (`adb shell input swipe x y x y 150`); no `input text`. Show touches was on during the recording.

Target: 2551Q, Schedule 1, row 1, Taxable Amount. Before proof is `try/android-money-box-1` (`docs/proof/android-money-box/README.md`).

Result at `1988536bfd41b22ee77d6f1e39ef29a61d4bbc87`:

- Zero amounts are empty while focused, so the first digit does not go before `0.00`.
- Keypad taps produced exactly `99999999.99`, one edit at a time, with the caret after the edit.
- Done/blur formatted and saved it as `99,999,999.99`; the row showed `PT010 · 99,999,999.99 · 3.0 · 3,000,000.00` and the page showed `✓ Saved`.

Files: `money-after.mp4`, `typed-99999999-99.png`, `blur-99999999-99.png`.

Gap: the development fixture used the test environment's scratch database, so reopening the draft in a new process did not preserve it. A persistent-database reopening proof is still required.
