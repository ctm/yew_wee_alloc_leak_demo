use {
    wasm_bindgen::prelude::*,
    yew::{prelude::*,html::Scope},
};

pub struct App {
    link: Scope<Self>,
    log_data: Vec<String>,
    lines: u32,
}

pub enum Msg {
    ReceivedData,
    ScrollLog,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::ReceivedData);
        Self {
            link,
            log_data: vec![],
            lines: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        const MAX_LINES: u32 = 1500;
        use Msg::*;

        match msg {
            ReceivedData => {
                self.log_data.push("When in the course of human events".to_string());
                self.link.send_message(ScrollLog);
                self.lines += 1;
            }
            // NOTE: if we totally take out the ScrollLog messge, we balloon
            // up to 1,010 MB instead of 1.9 GB.  This implies the leak is
            // either due to the sending or the updating of the view...
            // However, if we return false, we also only go up to 1,010 MB.
            // So it's not due to sending and receiving the ScrollLog message,
            // but due to all the stuff that happens after we return true,
            // even though we haven't changed anything that will change
            // what view creates.
            ScrollLog => {
                if self.lines < MAX_LINES {
                    self.link.send_message(ReceivedData);
                }
                // If we return false here, we'll get about half as big
                // return false;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{
                for self.log_data.iter().map(|line| {
                    html! {
                        <span>
                            { line }
                            <br/>
                        </span>
                    }
                })
            }</p>
        }
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}
