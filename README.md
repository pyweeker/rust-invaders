Fixing original project to Bevy 0.6  : Work on progress

ideas : scroll and tiled maps ; parallax

example for GLSL background  available here : https://github.com/Bobox214/Kataster

Wednesay 12 th january:  **Does compile but 3 warning and a bug in the window: **

warning: unused variable: `materials`
   --> src/main.rs:126:6
    |
126 |     mut materials: ResMut<Assets<ColorMaterial>>,
    |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_materials`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
   --> src/main.rs:126:2
    |
126 |     mut materials: ResMut<Assets<ColorMaterial>>,
    |     ----^^^^^^^^^
    |     |
    |     help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: field is never read: `group_id`
  --> src/enemy.rs:21:2
   |
21 |     group_id: u32,
   |     ^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rust-invaders` (bin "rust-invaders") generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rust-invaders RUST_BACKTRACE=1`
2022-01-12T19:53:00.408206Z  WARN winit::platform_impl::platform::x11::util::randr: XRandR reported that the display's 0mm in size, which is certifiably insane    
2022-01-12T19:53:00.408897Z  INFO winit::platform_impl::platform::x11::window: Guessed window scale factor: 1    
MESA-INTEL: warning: Performance support disabled, consider sysctl dev.i915.perf_stream_paranoid=0

MESA-INTEL: warning: Performance support disabled, consider sysctl dev.i915.perf_stream_paranoid=0


