use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

pub enum AppMessage {}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _new_prop: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"Hello Yew!"}
            </div>
        }
    }
}
