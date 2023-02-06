let chunk = 80;
let layers = 5;
let pixelate = 1.;

@group(1) @binding(0)
var<uniform> player_position: vec2<f32>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};

fn hash(p: vec2<f32>) -> f32 {
    var p = fract(p * vec2<f32>(123.312934, 456.21231));
    p += dot(p, p + 45.271902);
    return fract(p.x * p.y);
}

fn single_star(gr: vec2<f32>, id: vec2<i32>) -> vec3<f32> {
    let hash_y = hash(vec2<f32>(id));

    // No star
    if hash_y < 0.98 {
        return vec3<f32>(0.);
    }

    let hash_x = hash(vec2<f32>(id) + vec2<f32>(198., 91.));

    let c = vec2<f32>(gr.x + hash_x - 0.5, gr.y + hash_y - 0.5);

    let dist = length(c);
    let glow = 0.04 / dist;

    let ray = max(0.0, 1.0 - abs(c.x * c.y * 800.0)) * smoothstep(0.4, 0.1, dist);

    let value = smoothstep(.01, 1., glow + ray);

    let shade = vec3(sin(hash_y * 7.), sin(hash_x * 7.), sin(hash_x * 31. + hash_y * .31));

    return shade * value;
}

fn stars(gr: vec2<f32>, id: vec2<i32>) -> vec3<f32> {
    var sum_color: vec3<f32>;

    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<i32>(x, y);

            sum_color += single_star(gr - vec2<f32>(offset), id + offset);
        }
    }
    return sum_color;
}

fn star_layer(pos: vec2<f32>, player_pos: vec2<f32>, layer: i32) -> vec3<f32> {
    let dl = f32(layer) / f32(layers);
    let local_chunk = f32(chunk) * dl;
    let relpos = pos + (dl + 0.5) * player_pos * 10.;
    let id: vec2<i32> = vec2<i32>(relpos / f32(local_chunk)) + 300 * layer;
    let gr = fract(relpos / f32(local_chunk)) - 0.5;

    return stars(gr, id);
}

@fragment
fn fragment(in: FragmentInput, @builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    var col = vec3<f32>(0.);

    for (var layer = 0; layer < layers; layer++) {
        col += star_layer(position.xy, player_position, layer);
    }

    let color = vec4<f32>(col, 1.0);

    return color;
}
