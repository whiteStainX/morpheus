# Bug Log

## 2025-10-21: Unresponsive Input Handling for Player Movement

**Problem:**
The player-controlled square in `MyTestScene` does not respond correctly to arrow key presses. Holding a key (e.g., `Right`) causes continuous movement that cannot be stopped or counteracted by pressing the opposite key (e.g., `Left`). The behavior is erratic and not as expected.

**Analysis:**
The root cause appears to be related to how `InputState` is being managed across frames within the main loop in `crates/cli/src/main.rs`. The state is likely being reset or not updated correctly, leading to `is_key_pressed` returning stale or incorrect values.

**Status:**
- **Open**

**Next Steps:**
- Re-examine the main loop in `run_scene` in `crates/cli/src/main.rs`.
- Ensure that a single, persistent `InputState` instance is created before the loop starts.
- Verify that `input_state.poll_events()` is called exactly once per frame.
- Confirm that the same `input_state` instance is passed to the `Context` for both the `on_update` and `on_draw` calls.
