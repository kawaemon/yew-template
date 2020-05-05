use yew::{Component, ComponentLink, ShouldRender, html, Html};

pub struct App {
    Link: ComponentLink<Self>
}

pub enum AppMessage {}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, Link: ComponentLink<Self>) -> Self {
        App { Link }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"Hello Yew!"}
            </div>
        }
    }
}
