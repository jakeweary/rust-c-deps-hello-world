use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::time::Duration;

use scopeguard::defer;

mod c;

fn main() {
  unsafe {
    c::glfwInit();
    defer!(c::glfwTerminate());

    c::glfwWindowHint(c::GLFW_CONTEXT_VERSION_MAJOR as c_int, 4);
    c::glfwWindowHint(c::GLFW_CONTEXT_VERSION_MINOR as c_int, 6);
    c::glfwWindowHint(c::GLFW_OPENGL_PROFILE as c_int, c::GLFW_OPENGL_CORE_PROFILE as c_int);

    let window_title = CString::new("hello world").unwrap();
    let window = c::glfwCreateWindow(640, 360, window_title.as_ptr(), null_mut(), null_mut());
    defer!(c::glfwDestroyWindow(window));

    c::glfwMakeContextCurrent(window);
    c::glfwSwapInterval(1);
    c::gladLoadGL(Some(c::glfwGetProcAddress));

    // https://github.com/rust-lang/rust-bindgen/issues/753
    c::glad_glClearColor.unwrap()(0.7, 0.9, 0.1, 1.0);
    c::glad_glClear.unwrap()(c::GL_COLOR_BUFFER_BIT);

    c::glfwSwapBuffers(window);

    std::thread::sleep(Duration::from_secs(1));
  }
}
