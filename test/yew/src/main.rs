use gloo_console::log;
use yew::{html, Component, Context, Html};

enum Action {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Action;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, m: Self::Message) -> bool {
        match m {
            Action::AddOne => {
                self.value += 1;
                log!(&self.value.to_string());
                // 重新渲染组件
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button onclick={ctx.link().callback(|_| Action::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
