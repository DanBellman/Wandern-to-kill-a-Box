#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}

@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

// Simple noise function for sparkles
fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

// Generate sparkles using noise
fn sparkles(uv: vec2<f32>, time: f32, density: f32) -> f32 {
    let scaled_uv = uv * density;
    let grid_uv = floor(scaled_uv);
    let local_uv = fract(scaled_uv);

    // Create animated sparkle
    let sparkle_time = time * 3.0;
    let sparkle_hash = hash(grid_uv + floor(sparkle_time));
    let sparkle_phase = fract(sparkle_hash + sparkle_time);

    // Make sparkles appear and fade
    let sparkle_intensity = smoothstep(0.7, 0.8, sparkle_hash) *
                           (1.0 - smoothstep(0.8, 1.0, sparkle_phase)) *
                           smoothstep(0.0, 0.2, sparkle_phase);

    // Create star shape for sparkles
    let center_dist = length(local_uv - 0.5);
    let star_shape = max(
        abs(local_uv.x - 0.5) * 2.0,
        abs(local_uv.y - 0.5) * 2.0
    );
    let sparkle_shape = smoothstep(0.4, 0.2, star_shape);

    return sparkle_intensity * sparkle_shape;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the base texture (CoinBox sprite)
    let base_color = textureSample(base_color_texture, base_color_sampler, in.uv);

    let time = globals.time;

    // More prominent shimmering colors (golden/silver)
    let shimmer_speed = 3.5;
    let shimmer_wave = sin(time * shimmer_speed + in.uv.x * 12.0 + in.uv.y * 10.0) * 0.5 + 0.5;
    let shimmer_wave2 = cos(time * shimmer_speed * 0.7 + in.uv.x * 15.0 - in.uv.y * 8.0) * 0.5 + 0.5;

    // Much stronger shimmer tint
    let shimmer_intensity = 0.8;
    let gold_tint = vec3<f32>(1.6, 1.4, 0.6);
    let silver_tint = vec3<f32>(1.3, 1.3, 1.8);
    let rainbow_tint = vec3<f32>(1.5, 1.2, 1.7);
    let shimmer_tint = mix(mix(gold_tint, silver_tint, shimmer_wave), rainbow_tint, shimmer_wave2 * 0.5);

    // More prominent sparkles
    let sparkle1 = sparkles(in.uv, time, 8.0);
    let sparkle2 = sparkles(in.uv + vec2<f32>(0.5), time * 1.3, 10.0);
    let sparkle3 = sparkles(in.uv + vec2<f32>(0.3, 0.7), time * 0.8, 12.0);
    let total_sparkles = sparkle1 + sparkle2 * 0.8 + sparkle3 * 0.6;

    // Apply strong shimmer tint to base color
    var final_color = base_color.rgb * mix(vec3<f32>(1.0), shimmer_tint, shimmer_intensity);

    // Add very bright sparkles on top
    final_color += vec3<f32>(total_sparkles * 1.2);

    // Add extra glow effect
    let glow = sin(time * 4.0) * 0.1 + 0.1;
    final_color += vec3<f32>(glow * 0.3, glow * 0.2, glow * 0.4);

    return vec4<f32>(final_color, base_color.a);
}
