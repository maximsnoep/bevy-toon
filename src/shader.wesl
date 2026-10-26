import bevy_pbr::render::forward_io::VertexOutput;
import bevy_pbr::render::mesh_view_bindings::view;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = in.color;

    let normal = normalize(in.world_normal);
    // Lit from the camera: the direction to its eye, per fragment.
    let view_dir = normalize(view.world_position - in.world_position.xyz);
    let n_dot_l = dot(normal, view_dir);

    // Clamp to non-negative lighting
    let n_dot_l_clamped = max(n_dot_l, 0.0);

    // Toon-style banding
    let bands = 3.0;
    var x = n_dot_l_clamped * bands;
    x = round(x);
    var light_intensity = x / bands;

    // Raise the minimum brightness to 0.6
    let min_brightness = 0.6;
    light_intensity = light_intensity * (1.0 - min_brightness) + min_brightness;

    // Apply lighting to vertex color
    let final_color = base_color.rgb * light_intensity;

    return vec4(final_color, 1.0);
}