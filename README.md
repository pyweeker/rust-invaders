Fixing original project to Bevy 0.6  : Work on progress

Wednesay 12 th january:
error[E0277]: the trait bound `bevy::prelude::Vec2: From<bevy::prelude::Vec3>` is not satisfied
   --> src/main.rs:173:22
    |
173 |             let laser_scale = Vec2::from(laser_tf.scale);
    |                               ^^^^^^^^^^ the trait `From<bevy::prelude::Vec3>` is not implemented for `bevy::prelude::Vec2`
    |
    = help: the following implementations were found:
              <bevy::prelude::Vec2 as From<(f32, f32)>>
              <bevy::prelude::Vec2 as From<XY<f32>>>
              <bevy::prelude::Vec2 as From<[f32; 2]>>
              <bevy::prelude::Vec2 as From<mint::vector::Point2<f32>>>
              <bevy::prelude::Vec2 as From<mint::vector::Vector2<f32>>>
note: required by `std::convert::From::from`

________________________
Simple RUST Game using the [Bevy Engine](https://bevyengine.org/)

Fast dev: `cargo watch -q -c -x 'run --features bevy/dynamic'`

- YouTube videos for this code base:
    - [Episode 1](https://youtu.be/Yb3vInxzKGE) - Rust Game Development tutorial from Scratch with Bevy Engine
    - [Episode 2](https://youtu.be/Dl4PJG0eRhg) - bi-lasers, enemy spawn, explosion animation
    - [Episode 3](https://youtu.be/4nEUX2hf2ZI) - Enemy Approach, Formation, and Laser
- Other Rust videos:
    - Weekly Rust Videos at [Jeremy Chone](https://www.youtube.com/jeremychone) channel
    - [Rust Game Development Tutorials](https://youtube.com/playlist?list=PL7r-PXl6ZPcCB_9zZFU0krBoGK3y5f5Vt)


## Rust & Bevy & Other Resources

- Rust: 
    - [Rust Book](https://doc.rust-lang.org/book/)
    - [Rust Video Course](https://www.youtube.com/playlist?list=PL7r-PXl6ZPcB4jn1_VR3D8tSK9DxOaiQE)
- Bevy: 
    - [Official Bevy Web Site](https://bevyengine.org/)
    - [Official Bevy Book](https://bevyengine.org/learn/book/introduction/)
    - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
    - [Official Bevy API Doc](https://docs.rs/bevy/0.5.0/bevy/)
    - [Official Bevy Assets](https://bevyengine.org/assets/)
    - [Offical GitHub examples](https://github.com/bevyengine/bevy/tree/latest/examples)
    - [Great Blog Post - snake game](https://mbuffett.com/posts/bevy-snake-tutorial/)
- Assets: 
    - [Player, Laser, Enemy Sprites](https://opengameart.org/content/space-shooter-redux)
    - [Explosion](https://opengameart.org/content/explosion)    
