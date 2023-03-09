use std::mem::MaybeUninit;

use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "requestAnimationFrame")]
    pub fn request_animation_frame(callback: JsValue) -> u32;
    #[wasm_bindgen(js_name = "cancelAnimationFrame")]
    pub fn cancel_animation_frame(id: u32);
}

pub fn register_animation_frame<T: 'static + FnMut(Option<f64>) -> bool>(mut frame: T) {
    struct C {
        function_rawbox: *mut dyn FnMut(f64),
        function_js: JsValue,
    }
    impl Drop for C {
        fn drop(&mut self) {
            drop(unsafe { Box::from_raw(self.function_rawbox) });
        }
    }
    let closure = Box::<MaybeUninit<C>>::new(MaybeUninit::uninit());
    let closure: *mut _ = Box::into_raw(closure);
    let function: Box<dyn FnMut(f64)> = Box::new({
        let mut last_time = None;
        move |time_stamp: f64| {
            if frame(last_time.map(|p| time_stamp - p)) {
                last_time = Some(time_stamp);
                request_animation_frame(unsafe {
                    (*closure).assume_init_ref().function_js.clone()
                });
            } else {
                unsafe {
                    (*closure).assume_init_drop();
                    drop(Box::from_raw(closure));
                };
            }
        }
    });

    let function_rawbox = Box::into_raw(function);
    let function = Closure::wrap(unsafe { Box::from_raw(function_rawbox) });
    let function_js = function.into_js_value();
    unsafe {
        (*closure).write(C {
            function_rawbox,
            function_js: function_js.clone(),
        });
    }

    request_animation_frame(function_js);
}
