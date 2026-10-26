[![Crates.io](https://img.shields.io/crates/v/bevy_toon)](https://crates.io/crates/bevy_toon)

# Bevy Toon

A tiny toon shader for [Bevy](https://bevyengine.org): flat bands of light, lit from the view direction, in the vertex
colors of the mesh.

## How to use

Add the `ToonPlugin`, and give your meshes a `ToonMaterial` with the direction you look from:
```rust
App::new().add_plugins(ToonPlugin);

commands.spawn((
  Mesh3d(mesh),
  MeshMaterial3d(materials.add(ToonMaterial { view_dir: Vec3::Z })),
));
```

## Version Compatibility

| bevy | bevy_toon |
|------|-----------|
| 0.18.0 | 0.2.x |
| 0.20.0 | 0.3.x |
