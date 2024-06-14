#![allow(non_upper_case_globals)]
mod sorting;

extern crate glfw;
use glfw::{
    Context, 
    Key, 
    Action,
    GlfwReceiver
};

extern crate gl;
use gl::types::*;

use std::{cmp,
    sync::mpsc,
    thread,
    ptr,
    usize,
    io::prelude::*,
    ffi::CString,
    fs::File,
    mem,
    os::raw::c_void
};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[allow(non_snake_case)]
pub fn main() {

    // glfw: initialize and configure
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // glfw window creation
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "sortingAlgs", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shaderProgram= unsafe {
        let (fragmentShaderSource, vertexShaderSource) = {
            let mut buf_fragment = Vec::new();
            let mut buf_verticies = Vec::new();

            File::open("fragment.glsl").expect("cannot find fragment shaders")
                .read_to_end(&mut buf_fragment).expect("no fragment shaders");
            File::open("verteces.glsl").expect("cannot find vertex shaders")
                .read_to_end(&mut buf_verticies).expect("no vertex shaders");
                
            (CString::from_vec_unchecked(buf_fragment), CString::from_vec_unchecked(buf_verticies))
        };
        // build and compile our shader program
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        // fragment shader
        let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragmentShader);

        // link shaders
        let shaderProgram = gl::CreateProgram();
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);

        gl::DeleteShader(vertexShader);
        gl::DeleteShader(fragmentShader);


        shaderProgram
    };

    let (tx, rx) = mpsc::channel();
    let (tax, rax) = mpsc::channel();


    thread::spawn(move | | sorting::sort(tx, tax));
    let mut i = 1;
    let mut arr = Vec::new();
    // render loop
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            //code for bars
            arr = rax.try_recv().unwrap_or(arr);
            i = rx.try_recv().unwrap_or(i); // TODO: add support for multiple pointers
            drawBars(&arr,shaderProgram, i);
            window.swap_buffers();
            glfw.poll_events();
            // draw our first triangle
            //gl::UseProgram(shaderProgram);
            //gl::BindVertexArray(getVertArr(buildBars(vec![1,2,3,4]))); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            //gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            
            //bazinga(shaderProgram);
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

fn error_callback(err: glfw::Error, description: String) {
    println!("GLFW error {:?}: {:?}", err, description);
}

fn read_shaders() -> (CString, CString){
    
}

fn get_max(arr:&Vec<i32>) -> i32 {
    let mut max = 0;
    for i in arr {
        max = cmp::max(max, *i);
    }
    max
}

#[allow(non_snake_case)]
unsafe fn drawBars(array: &Vec<i32>, shaderProgram:u32, arrPointer:usize) -> (){
    
    let gaps = (1.0-1.0/array.len() as f32)/array.len() as f32;
    let width:f32 = 1.0/array.len() as f32;
    
    let vStretch:f32 = 2.0*(1.0-gaps)/get_max(array) as f32;
    
    gl::UseProgram(shaderProgram);
    let mut VAO = vec![0;array.len()];
    for i in 0..array.len() {
        let bar_pos_x = i as f32 * width + (i as f32 + 1.0) * gaps;
        let bar_height = array[i] as f32 * vStretch;
        let vertices: [f32;8] = [
            -1.0 + bar_pos_x,
            -1.0 + gaps,
            -1.0 + bar_pos_x + width,
            -1.0 + gaps,
            -1.0 + bar_pos_x + width,
            -1.0 + bar_height + gaps,
            -1.0 + bar_pos_x,
            -1.0 + bar_height + gaps
        ];
  
        VAO[i] = createVAO(vertices, if i == arrPointer {[1.0, 0.0, 0.0]} else {[1.0, 1.0, 1.0]});
    }

    for i in VAO {
        gl::BindVertexArray(i);
        gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
    } 
}

#[allow(non_snake_case)]
unsafe fn createVAO(vertices:[f32;8], color:[f32;3]) -> u32 {
    let (mut VAO, mut VBO) = (0, [0,0]);
    let colors:[f32;12] = [color[0], color[1], color[2], color[0], color[1], color[2], color[0], color[1], color[2], color[0], color[1], color[2]];

    gl::GenBuffers(2, &mut VBO[0]);
    gl::GenVertexArrays(1, &mut VAO);

    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO[1]);
    gl::BufferData(gl::ARRAY_BUFFER,
        (colors.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, 
        &colors[0] as *const f32 as *const c_void, 
        gl::STATIC_DRAW);
    gl::VertexAttribPointer(1 as u32, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
    gl::EnableVertexAttribArray(1 as u32);


    gl::BindBuffer(gl::ARRAY_BUFFER, VBO[0]);

    gl::BufferData(gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void, 
        gl::STATIC_DRAW);
    gl::VertexAttribPointer(0 as u32, 2, gl::FLOAT, gl::FALSE, 2 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
    gl::EnableVertexAttribArray(0 as u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0 as u32);
    VAO
}