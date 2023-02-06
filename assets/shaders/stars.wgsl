let chunk = 160;
let layers = 5.;
// let pixelate = 1.;
let offset_max = 0.5;
let shade_base = vec3<f32>(0.0, 0.0, 0.0);
let shade_add = vec3<f32>(1.0, 1.0, 1.0);
let star_chance: f32 = 0.05;

@group(1) @binding(0)
var<uniform> player_position: vec2<f32>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
};

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2(12.9898, 4.1414))) * 43758.5434);
}

fn single_star(gr: vec2<f32>, id: vec2<f32>) -> vec3<f32> {
    let h = hash(id);

    if h > star_chance {
        return vec3(0.);
    }

    let hash_y = hash(id + vec2(13., 123.));

    let hash_x = hash(id + vec2(198., 91.));

    let hash_offset = vec2(hash_x, hash_y) * offset_max - vec2(offset_max) / 2.;
    let c = gr + hash_offset;

    let dist = length(c);
    let glow = 0.01 / dist;

    let ray = max(0.0, 1.0 - abs(c.x * c.y * 3000.0)) * smoothstep(0.1, 0., dist);

    let value = smoothstep(.02, 1., glow + ray);

    let shade_mix = vec3(sin(hash_x * 9.1798), sin(hash_x * 7.321), sin(hash_x * 31. + hash_y * 3.1));
    let shade = shade_base + shade_add * shade_mix / 2. + shade_add * 2.;

    return shade * value;
}

fn star_layer(pos: vec2<f32>, player_pos: vec2<f32>, layer: f32) -> vec3<f32> {
    let dl = layer / layers;
    let local_chunk = f32(chunk) * dl;
    let layer_shift = vec2((layer + 1233.) * 312.412, (layer + 123.) * 1235.76);
    let relpos = pos + (dl + 0.5) * player_pos + layer_shift;
    // let pixelated = pixelate * floor(relpos / pixelate); // replace relpos with pixelated
    let id = floor(relpos / f32(local_chunk)); // the grid position, whole numbers
    let gr = fract(relpos / f32(local_chunk)) - 0.5; // the subposition in the grid, fractionals

    let brightness = (layer / layers);

    return single_star(gr, id) * brightness;
}

@fragment
fn fragment(in: FragmentInput, @builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    var col = vec3<f32>(0.);

    for (var layer = 1.; layer <= layers; layer += 1.) {
        col += star_layer(position.xy, player_position, layer) * (layer / layers);
    }

    let color = vec4<f32>(col, 1.0);

    return color;
}

// vim: ft=wgsl
