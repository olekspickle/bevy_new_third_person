
# WIP: this template is just an attempt to create simple and up to date template
# Simple Third Person Bevy game template

Template for a Game using the awesome [Bevy] engine featuring out of the box builds for Windows, Linux, macOS, Web (Wasm)
This template is a great way to get started on a new 3D [Bevy] game!
Start with a [basic project](#write-your-game) and [CI / CD](#release-your-game) that can deploy to [itch.io](https://itch.io).
You can [try this template in your browser!](https://olekspickle.itch.io/bevy-third-person)

# Best way to start

Install [cargo-generate] and run:
```bash
cargo generate olekspickle/bevy_new_third_person
```

# Write your game

The best way to get started is to play around with the code you find in [`src/game/`](./src/game).
This template comes with a basic project structure that you may find useful:

| Path                                               | Description                                                        |
| -------------------------------------------------- | ------------------------------------------------------------------ |
| [`src/main.rs`](./src/main.rs)                     | App entrypoint                                                     |
| [`src/lib.rs`](./src/lib.rs)                       | App setup                                                          |
| [`src/loading/`](./src/loading)                    | A high-level way to load collections of asset handles as resources |
| [`src/audio.rs`](./src/audio.rs)                   | Marker components for sound effects and music                      |
| [`src/game/`](./src/game)                          | Example game mechanics & content (replace with your own code)      |
| [`src/dev_tools.rs`](./src/dev_tools.rs)           | Dev tools for dev builds (press \` aka backtick to toggle)         |
| [`src/screens/`](./src/screens)                    | Splash screen, title screen, gameplay screen, etc.                 |
| [`src/ui/`](./src/ui)                              | Reusable UI widgets & theming                                      |

Feel free to move things around however you want, though.

# Features:
- [x] import and usage of game mechanics and parameters from .ron config
- [x] simple asset loading from [BevyFlock] example with loading from path addition
- [x] third person camera with [bevy_third_person_camera]
- [x] simple key mapping to game actions using [leafwing-input-manager]
- [x] simple scene with colliders and rigid bodies using [avian3d]
- [x] simple player movement using [bevy_tnua]
- [x] simple skybox sun cycle using [bevy atmosphere example]
- [x] rig and animations using [Universal Animation Library] from quaternius
- [ ] experimental sound with [bevy_seedling] based on Firewheel audio engine (which will probably replace bevy_audio)
- [ ] vault mechanics
- [ ] small door/portal demo

# Run your game

WARNING: if you work in a private repository, please be aware that macOS and Windows runners cost more build minutes.
**For public repositories the workflow runners are free!**

# Known issues

Audio in web-builds can have issues in some browsers. This seems to be a general performance issue with wasm builds in all engines, the best solution is just to artificially extend loading phase(seems to be a solution people go for in other engines)

# License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory (see [Credits](credits/CREDITS.md)).

[android-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup
[avian3d]: https://github.com/Jondolf/avian/tree/main/crates/avian3d
[bevy]: https://bevyengine.org/
[bevy atmosphere example]: https://bevyengine.org/examples/3d-rendering/atmosphere/
[bevy-discord]: https://discord.gg/bevy
[bevy-learn]: https://bevyengine.org/learn/
[bevy_third_person_camera]: https://github.com/The-DevBlog/bevy_third_person_camera
[bevy_tnua]: https://github.com/idanarye/bevy-tnua
[bevy_seedling]: https://github.com/CorvusPrudens/bevy_seedling
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[BevyFlock]: https://github.com/TheBevyFlock/bevy_new_2d
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[ios-instructions]: https://github.com/bevyengine/bevy/blob/latest/examples/README.md#setup-1
[leafwing-input-manager]: https://github.com/Leafwing-Studios/leafwing-input-manager
[mobile_dev_with_bevy_2]: https://www.nikl.me/blog/2023/notes_on_mobile_development_with_bevy_2/
[trunk]: https://trunkrs.dev/
[Universal Animation Library]: https://quaternius.itch.io/universal-animation-library
[workflow_bevy_android]: https://www.nikl.me/blog/2023/github_workflow_to_publish_android_app/
[workflow_bevy_ios]: https://www.nikl.me/blog/2023/github_workflow_to_publish_ios_app/
