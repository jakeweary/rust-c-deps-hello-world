use std::ffi::CString;
use std::ptr::null_mut;

use scopeguard::defer;

mod c;

fn main() {
  unsafe {
    c::glfwInit();
    defer!(c::glfwTerminate());

    c::glfwWindowHint(c::GLFW_CONTEXT_VERSION_MAJOR, 4);
    c::glfwWindowHint(c::GLFW_CONTEXT_VERSION_MINOR, 6);
    c::glfwWindowHint(c::GLFW_OPENGL_PROFILE, c::GLFW_OPENGL_CORE_PROFILE);

    let window_title = CString::new("hello world").unwrap();
    let window = c::glfwCreateWindow(640, 360, window_title.as_ptr(), null_mut(), null_mut());
    defer!(c::glfwDestroyWindow(window));

    c::glfwMakeContextCurrent(window);
    c::glfwSwapInterval(1);
    c::gladLoadGL(Some(c::glfwGetProcAddress));

    while c::glfwWindowShouldClose(window) == c::GLFW_FALSE {
      // https://github.com/rust-lang/rust-bindgen/issues/753
      c::glad_glClearColor.unwrap()(0.7, 0.9, 0.1, 1.0);
      c::glad_glClear.unwrap()(c::GL_COLOR_BUFFER_BIT);

      c::glfwSwapBuffers(window);
      c::glfwPollEvents();
    }
  }
}
