use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::WHITE,
    material::{Material, MaterialParams, gl_use_default_material, gl_use_material, load_material},
    math::vec2,
    miniquad::{ShaderSource, UniformDesc, UniformType},
    texture::{
        DrawTextureParams, FilterMode, RenderTarget, Texture2D, draw_texture_ex, render_target,
    },
    window::{screen_height, screen_width},
};

const VERTEX: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";

const H_BLUR_FRAG: &str = "#version 100
precision mediump float;
varying lowp vec2 uv;
uniform sampler2D Texture;
uniform vec2 tex_size;
void main() {
    float sigma = 8.0;
    vec4 sum = vec4(0.0);
    float total = 0.0;
    for (int i = -12; i <= 12; i++) {
        float fi = float(i);
        float weight = exp(-0.5 * fi * fi / (sigma * sigma));
        sum += texture2D(Texture, uv + vec2(fi / tex_size.x, 0.0)) * weight;
        total += weight;
    }
    gl_FragColor = sum / total;
}
";

const V_BLUR_FRAG: &str = "#version 100
precision mediump float;
varying lowp vec2 uv;
uniform sampler2D Texture;
uniform vec2 tex_size;
void main() {
    float sigma = 8.0;
    vec4 sum = vec4(0.0);
    float total = 0.0;
    for (int i = -12; i <= 12; i++) {
        float fi = float(i);
        float weight = exp(-0.5 * fi * fi / (sigma * sigma));
        sum += texture2D(Texture, uv + vec2(0.0, fi / tex_size.y)) * weight;
        total += weight;
    }
    gl_FragColor = sum / total;
}
";

pub struct BlurPipeline {
    h_target: RenderTarget,
    h_material: Material,
    v_material: Material,
}

impl BlurPipeline {
    pub fn new() -> Self {
        let sw = screen_width() as u32;
        let sh = screen_height() as u32;
        let h_target = render_target(sw.max(1), sh.max(1));
        h_target.texture.set_filter(FilterMode::Linear);

        let uniforms = vec![UniformDesc::new("tex_size", UniformType::Float2)];

        let h_material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: H_BLUR_FRAG,
            },
            MaterialParams {
                uniforms: uniforms.clone(),
                ..Default::default()
            },
        )
        .expect("Failed to load horizontal blur material");

        let v_material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: V_BLUR_FRAG,
            },
            MaterialParams {
                uniforms,
                ..Default::default()
            },
        )
        .expect("Failed to load vertical blur material");

        Self {
            h_target,
            h_material,
            v_material,
        }
    }

    /// Applies a two-pass Gaussian blur to `source` and draws the result to the current render target.
    pub fn apply(&mut self, source: &Texture2D, screen_w: f32, screen_h: f32) {
        let w = (screen_w as u32).max(1);
        let h = (screen_h as u32).max(1);

        if self.h_target.texture.width() as u32 != w || self.h_target.texture.height() as u32 != h {
            self.h_target = render_target(w, h);
            self.h_target.texture.set_filter(FilterMode::Linear);
        }

        // Pass 1: horizontal blur — source → h_target
        set_camera(&Camera2D {
            render_target: Some(self.h_target.clone()),
            zoom: vec2(2.0 / screen_w, -2.0 / screen_h),
            target: vec2(screen_w / 2.0, screen_h / 2.0),
            ..Default::default()
        });
        gl_use_material(&self.h_material);
        self.h_material
            .set_uniform("tex_size", (screen_w, screen_h));
        draw_texture_ex(
            source,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
        set_default_camera();

        // Pass 2: vertical blur — h_target → current render target (screen)
        gl_use_material(&self.v_material);
        self.v_material
            .set_uniform("tex_size", (screen_w, screen_h));
        draw_texture_ex(
            &self.h_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
