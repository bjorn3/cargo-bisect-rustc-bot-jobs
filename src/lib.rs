use std::sync::{Arc, RwLock};

#[derive(Copy, Clone)]
pub struct Glfw;

fn fun_call<T>(val: T) -> T { val }

static mut GLFW: Option<Glfw> = None;
pub fn new() -> Result<Arc<RwLock<Glfw>>, Box<dyn std::error::Error>> {
    //TODO: Give the option to choose FAIL / LOG?
    let glfw = unsafe { if let Some(glfw) = GLFW {
        glfw
    } else {
        let glfw = fun_call(Glfw);
        GLFW = Some(glfw);
        glfw
    }};

    Ok(Arc::new(RwLock::new(glfw)))
}