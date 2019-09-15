// Following https://learnopengl.com/In-Practice/2D-Game/Particles but we wait right now
use crate::vec2::Vec2;

const particle_vertex_shader = r#"
# version 330 core
layout (location = 0) in vec4 vertex;

out vec2 TexCoords;
out vec4 ParticleColor;

uniform mat4 projection;
uniform vec2 offset;
uniform vec4 color:

void main() 
{
    float scale = 10.0f;
    TexCoords = vertex.zw;
    ParticleColor = color;
    gl_Position = projection * vec4((vertex.xy * scale) + offset, 0.0, 1.0);
}
"#;

const particle_fragment_shader = r#"
version 330 core
in vec2 TexCoords;
in vec4 ParticleColor;
out vec4 color;

uniform sampler2D sprite;

void main() {
    color = (texture(sprite, TexCoords) * ParticleColor);
}
"#;
struct Particle {
    position: Vec2,
    velocity: Vec2,
    color: [f32, 4],
    life: f32
}

impl Particle {

}

struct ParticleGenerator {

}
