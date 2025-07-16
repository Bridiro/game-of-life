use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
};
extern crate js_sys;

#[wasm_bindgen]
pub struct GameOfLife {
    gl: WebGlRenderingContext,
    compute_program: WebGlProgram,
    render_program: WebGlProgram,
    current_state: WebGlTexture,
    next_state: WebGlTexture,
    quad_buffer: WebGlBuffer,
    framebuffer: WebGlFramebuffer,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl GameOfLife {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, width: u32, height: u32) -> Result<GameOfLife, JsValue> {
        let gl = init_webgl_context(canvas_id)?;

        let current_state = create_texture(&gl, width, height)?;
        let next_state = create_texture(&gl, width, height)?;

        let framebuffer = gl
            .create_framebuffer()
            .ok_or_else(|| JsValue::from_str("Failed to create framebuffer"))?;

        let quad_buffer = create_quad_buffer(&gl)?;

        let compute_program = create_compute_program(&gl)?;
        let render_program = create_render_program(&gl)?;

        let game = GameOfLife {
            gl,
            compute_program,
            render_program,
            current_state,
            next_state,
            quad_buffer,
            framebuffer,
            width,
            height,
        };

        Ok(game)
    }

    #[wasm_bindgen]
    pub fn randomize(&mut self) -> Result<(), JsValue> {
        let mut data = vec![0u8; (self.width * self.height * 4) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 4) as usize;

                let is_alive = js_sys::Math::random() < 0.3;
                let color = if is_alive { 255u8 } else { 0u8 };

                data[index] = color;
                data[index + 1] = color;
                data[index + 2] = color;
                data[index + 3] = 255;
            }
        }

        self.upload_texture_data(data)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) -> Result<(), JsValue> {
        let data = vec![0u8; (self.width * self.height * 4) as usize];
        self.upload_texture_data(data)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_glider(&mut self) -> Result<(), JsValue> {
        let mut data = vec![0u8; (self.width * self.height * 4) as usize];

        let center_x = self.width / 2;
        let center_y = self.height / 2;

        let glider_pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

        for (dx, dy) in glider_pattern.iter() {
            let x = center_x + dx;
            let y = center_y + dy;
            if x < self.width && y < self.height {
                let index = ((y * self.width + x) * 4) as usize;
                data[index] = 255;
                data[index + 1] = 255;
                data[index + 2] = 255;
                data[index + 3] = 255;
            }
        }

        self.upload_texture_data(data)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_oscillator(&mut self) -> Result<(), JsValue> {
        let mut data = vec![0u8; (self.width * self.height * 4) as usize];

        let center_x = self.width / 2;
        let center_y = self.height / 2;

        for i in 0..3 {
            let x = center_x + i - 1;
            let y = center_y;
            if x < self.width && y < self.height {
                let index = ((y * self.width + x) * 4) as usize;
                data[index] = 255;
                data[index + 1] = 255;
                data[index + 2] = 255;
                data[index + 3] = 255;
            }
        }

        self.upload_texture_data(data)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_beacon(&mut self) -> Result<(), JsValue> {
        let mut data = vec![0u8; (self.width * self.height * 4) as usize];

        let center_x = self.width / 2;
        let center_y = self.height / 2;

        let beacon_pattern = [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];

        for (dx, dy) in beacon_pattern.iter() {
            let x = center_x + dx;
            let y = center_y + dy;
            if x < self.width && y < self.height {
                let index = ((y * self.width + x) * 4) as usize;
                data[index] = 255;
                data[index + 1] = 255;
                data[index + 2] = 255;
                data[index + 3] = 255;
            }
        }

        self.upload_texture_data(data)?;
        Ok(())
    }

    fn upload_texture_data(&mut self, data: Vec<u8>) -> Result<(), JsValue> {
        self.current_state = self
            .gl
            .create_texture()
            .ok_or_else(|| JsValue::from_str("Failed to create texture"))?;

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.current_state));

        let result = self
            .gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&data),
            );

        if let Err(e) = result {
            return Err(e);
        }

        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        self.next_state = self
            .gl
            .create_texture()
            .ok_or_else(|| JsValue::from_str("Failed to create next_state texture"))?;

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.next_state));

        self.gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&data),
            )?;

        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, None);

        Ok(())
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<(), JsValue> {
        self.gl
            .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        self.gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.next_state),
            0,
        );

        self.gl
            .viewport(0, 0, self.width as i32, self.height as i32);
        self.gl.use_program(Some(&self.compute_program));

        self.gl.active_texture(WebGlRenderingContext::TEXTURE0);
        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.current_state));

        let texture_location = self
            .gl
            .get_uniform_location(&self.compute_program, "u_texture");
        self.gl.uniform1i(texture_location.as_ref(), 0);

        let resolution_location = self
            .gl
            .get_uniform_location(&self.compute_program, "u_resolution");
        self.gl.uniform2f(
            resolution_location.as_ref(),
            self.width as f32,
            self.height as f32,
        );

        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.quad_buffer));
        let position_location = self
            .gl
            .get_attrib_location(&self.compute_program, "a_position");
        self.gl.enable_vertex_attrib_array(position_location as u32);
        self.gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        std::mem::swap(&mut self.current_state, &mut self.next_state);

        Ok(())
    }

    #[wasm_bindgen]
    pub fn render(&self) -> Result<(), JsValue> {
        self.gl
            .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("gameCanvas")
            .unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        self.gl
            .viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
        self.gl.use_program(Some(&self.render_program));

        self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.gl.active_texture(WebGlRenderingContext::TEXTURE0);
        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.current_state));

        let texture_location = self
            .gl
            .get_uniform_location(&self.render_program, "u_texture");
        if texture_location.is_none() {
            return Err(JsValue::from_str("u_texture uniform not found"));
        }

        self.gl.uniform1i(texture_location.as_ref(), 0);

        self.gl
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.quad_buffer));
        let position_location = self
            .gl
            .get_attrib_location(&self.render_program, "a_position");
        if position_location == -1 {
            return Err(JsValue::from_str("a_position attribute not found"));
        }

        self.gl.enable_vertex_attrib_array(position_location as u32);
        self.gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        let error = self.gl.get_error();
        if error != WebGlRenderingContext::NO_ERROR {
            web_sys::console::log_1(&format!("WebGL error: {}", error).into());
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.width = width;
        self.height = height;

        self.current_state = create_texture(&self.gl, width, height)?;
        self.next_state = create_texture(&self.gl, width, height)?;

        self.randomize()?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn toggle_cell(&mut self, x: u32, y: u32) -> Result<(), JsValue> {
        if x >= self.width || y >= self.height {
            return Ok(());
        }

        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.current_state));

        let data = [255u8, 255, 255, 255];

        self.gl
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                1,
                1,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&data),
            )?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_cells_in_area(
        &mut self,
        center_x: u32,
        center_y: u32,
        radius: u32,
    ) -> Result<(), JsValue> {
        self.gl
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.current_state));

        let data = [255u8, 255, 255, 255];

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let x = center_x as i32 + dx;
                let y = center_y as i32 + dy;

                if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
                    if dx * dx + dy * dy <= (radius as i32) * (radius as i32) {
                        self.gl
                            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                                WebGlRenderingContext::TEXTURE_2D,
                                0,
                                x,
                                y,
                                1,
                                1,
                                WebGlRenderingContext::RGBA,
                                WebGlRenderingContext::UNSIGNED_BYTE,
                                Some(&data),
                            )?;
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.get_extension("OES_texture_float").ok();

    Ok(gl)
}

fn create_texture(
    gl: &WebGlRenderingContext,
    width: u32,
    height: u32,
) -> Result<WebGlTexture, JsValue> {
    let texture = gl
        .create_texture()
        .ok_or_else(|| JsValue::from_str("Failed to create texture"))?;

    gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));

    let data = vec![0u8; (width * height * 4) as usize];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGBA as i32,
        width as i32,
        height as i32,
        0,
        WebGlRenderingContext::RGBA,
        WebGlRenderingContext::UNSIGNED_BYTE,
        Some(&data),
    )?;

    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );

    Ok(texture)
}

fn create_quad_buffer(gl: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl
        .create_buffer()
        .ok_or_else(|| JsValue::from_str("Failed to create buffer"))?;

    let vertices: [f32; 12] = [
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
    ];

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok(buffer)
}

pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let error_log = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into());
        web_sys::console::log_1(&format!("Shader compilation error: {}", error_log).into());
        Err(JsValue::from_str(&error_log))
    }
}

fn create_compute_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = r#"
        attribute vec2 a_position;
        varying vec2 v_texCoord;
        
        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
            v_texCoord = a_position * 0.5 + 0.5;
        }
    "#;

    let fragment_shader_source = r#"
        precision mediump float;
        uniform sampler2D u_texture;
        uniform vec2 u_resolution;
        varying vec2 v_texCoord;
        
        int getCell(vec2 coord) {
            vec2 wrappedCoord = fract(coord);
            vec4 cell = texture2D(u_texture, wrappedCoord);
            return cell.r > 0.5 ? 1 : 0;
        }
        
        void main() {
            vec2 texelSize = 1.0 / u_resolution;
            vec2 coord = v_texCoord;
            
            int current = getCell(coord);
            
            int neighbors = 0;
            neighbors += getCell(coord + vec2(-1.0, -1.0) * texelSize);
            neighbors += getCell(coord + vec2( 0.0, -1.0) * texelSize);
            neighbors += getCell(coord + vec2( 1.0, -1.0) * texelSize);
            neighbors += getCell(coord + vec2(-1.0,  0.0) * texelSize);
            neighbors += getCell(coord + vec2( 1.0,  0.0) * texelSize);
            neighbors += getCell(coord + vec2(-1.0,  1.0) * texelSize);
            neighbors += getCell(coord + vec2( 0.0,  1.0) * texelSize);
            neighbors += getCell(coord + vec2( 1.0,  1.0) * texelSize);
            
            int newState = 0;
            if (current == 1) {
                if (neighbors == 2 || neighbors == 3) {
                    newState = 1;
                }
            } else {
                if (neighbors == 3) {
                    newState = 1;
                }
            }
            
            gl_FragColor = vec4(vec3(float(newState)), 1.0);
        }
    "#;

    let vertex_shader = create_shader(
        gl,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )?;
    let fragment_shader = create_shader(
        gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )?;

    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("Failed to create program"))?;

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(JsValue::from_str(
            &gl.get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ))
    }
}

fn create_render_program(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = r#"
        attribute vec2 a_position;
        varying vec2 v_texCoord;
        
        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
            v_texCoord = a_position * 0.5 + 0.5;
        }
    "#;

    let fragment_shader_source = r#"
        precision mediump float;
        uniform sampler2D u_texture;
        varying vec2 v_texCoord;
        
        void main() {
            vec4 texColor = texture2D(u_texture, v_texCoord);
            gl_FragColor = vec4(texColor.rgb, 1.0);
        }
    "#;

    let vertex_shader = create_shader(
        gl,
        WebGlRenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    )?;
    let fragment_shader = create_shader(
        gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        fragment_shader_source,
    )?;

    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("Failed to create program"))?;

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(JsValue::from_str(
            &gl.get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ))
    }
}
