# UBreakout-rs

**UBreakout-rs** is a learning purpose project. It is a simple game written in Rust with [`ggez`](https://github.com/ggez/ggez) library.

## To-do

- [ ] Scene management system
- [ ] Each scene have its own `draw` and `update` methods.
- [ ] Scenes must be created separated files and implement a scene trait.
- [ ] Main `game` should be responsible for global state, and calling for scenes.
- [ ] `Editor` scene, should be responsible for user be able to craete own levels.

## Notes

> [!WARNING]
> For now, there is no way to create custom levels (without editing JSON), I recommend copying the example_levels/ files to your levels directory (the game should show the correct path)
