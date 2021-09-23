use wasm_bindgen::JsValue;
use yew::web_sys::console::log_1;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

enum Action {
    AddOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Action;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, m: Self::Message) -> ShouldRender {
        match m {
            Action::AddOne => {
                self.value += 1;
                log_1(&JsValue::from_str(&self.value.to_string()));
                // 重新渲染组件
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Action::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
