 #![allow(non_upper_case_globals)]
 #[derive(Copy, Clone, Debug)]
    pub struct Attr_line {
        pub position_s: [f32; 2],
        pub position_e: [f32; 2],
        pub alpha: f32,
    }

implement_vertex!(Attr_line, position_s, position_e, alpha);

#[derive(Copy, Clone, Debug)]
    pub struct AttrCircle {
        pub position_c: [f32; 2],
        pub alpha: f32,
    }
implement_vertex!(AttrCircle, position_c, alpha);

pub const vertex_shader_src: &str = r#"
    #version 140

    in vec2 position;
    in vec2 position_c;
    in float alpha;
    out float v_alpha;

    void main() {
        v_alpha = alpha;
        gl_Position = vec4(position + position_c, 0.0, 1.0);
    }
    "#;

pub const fragment_shader_src: &str = r#"
    #version 140

    out vec4 color;
    in float v_alpha;

    void main() {
        color = vec4(0.0, 0.0, 0.0, v_alpha);
    }
    "#;
pub const vertex_line_shader_src: &str = r#"
    #version 140

    in int gl_VertexID; // we use the vertex Id to differentiate in the line where start is and where end
    in vec2 position;
    in vec2 position_s;
    in vec2 position_e;
    in float alpha;
    out float v_alpha;
    out vec2 pos_test;

    void main() {
        v_alpha = alpha;
        if (gl_VertexID == 0) {
        gl_Position = vec4(position + position_s, 0.0, 1.0);
        }
        else {
            gl_Position = vec4(position_e, 0.0, 1.0);
        }
    }
    "#;
pub const fragment_line_shader_src: &str = r#"
    #version 140
    in float v_alpha;
    out vec4 color;
    void main() {  
        color = vec4(0.0, 0.0, 0.0, v_alpha);
    }
    "#;
