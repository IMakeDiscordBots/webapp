use yew::prelude::*;

enum Msg {
    AddOne,
    AddTwo,
    AddThree,
    AddFour,
    AddFive,
    AddSix,
    AddSeven,
    AddEight,
    AddNine,
    AddTen,

}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self,  _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::AddTwo => {
                self.value += 2;
                true
            }
            Msg::AddThree => {
                self.value += 3;
                true
            }
            Msg::AddFour => {
                self.value += 4;
                true
            }
            Msg::AddFive => {
                self.value += 5;
                true
            }
            Msg::AddSix => {
                self.value += 6;
                true
            }
            Msg::AddSeven => {
                self.value += 7;
                true
            }
            Msg::AddEight => {
                self.value += 8;
                true
            }
            Msg::AddNine => {
                self.value += 9;
                true
            }
            Msg::AddTen => {
                self.value += 10;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link= ctx.link();
        html! {
            <>
                <div>
                    <button onclick = {link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <button onclick = {link.callback(|_| Msg::AddTwo)}>{ "+2" }</button>
                    <button onclick = {link.callback(|_| Msg::AddThree)}>{ "+3" }</button>
                    <button onclick = {link.callback(|_| Msg::AddFour)}>{ "+4" }</button>
                    <button onclick = {link.callback(|_| Msg::AddFive)}>{ "+5" }</button>
                    <button onclick = {link.callback(|_| Msg::AddSix)}>{ "+6" }</button>
                    <button onclick = {link.callback(|_| Msg::AddSeven)}>{ "+7" }</button>
                    <button onclick = {link.callback(|_| Msg::AddEight)}>{ "+8" }</button>
                    <button onclick = {link.callback(|_| Msg::AddNine)}>{ "+9" }</button>
                    <button onclick = {link.callback(|_| Msg::AddTen)}>{ "+10" }</button>
                    <p>{ self.value }</p>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
