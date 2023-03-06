use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
pub struct Drawing {
    canvas_ref: NodeRef,
    canvas_width: i32,
    canvas_height: i32,
}

pub enum Msg {
    PointerMove { x: i32, y: i32, dx: i32, dy: i32 },
}

impl Component for Drawing {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        Drawing {
            canvas_ref: NodeRef::default(),
            canvas_width: (body.client_width() as f64 * 0.8) as i32,
            canvas_height: (body.client_height() as f64 * 0.8) as i32,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PointerMove { x, y, dx, dy } => {
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let prev_x = x - dx;
                let prev_y = y - dy;

                context.begin_path();
                context.set_stroke_style(&JsValue::from("red"));
                context.set_line_cap("round");
                context.move_to(prev_x as f64, prev_y as f64);
                context.line_to(x as f64, y as f64);
                context.stroke();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onpointermove = ctx.link().batch_callback(|e: PointerEvent| {
            if !(e.is_primary() && e.pressure() > 0.0) {
                return None;
            }
            Some(Msg::PointerMove {
                x: e.offset_x(),
                y: e.offset_y(),
                dx: e.movement_x(),
                dy: e.movement_y(),
            })
        });

        html! {
            <canvas width={self.canvas_width.to_string()} height={self.canvas_height.to_string()} ref={&self.canvas_ref} {onpointermove}/>
        }
    }
}
