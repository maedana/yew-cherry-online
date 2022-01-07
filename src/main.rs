use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                log::info!("Clicked");
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
          <div>
            <nav class="navbar is-transparent">
              <div class="navbar-brand">
                <a class="navbar-item is-size-3" href="/">
                  { "Cherry Online" }
                </a>
                <div class="navbar-burger" data-target="navbarExampleTransparentExample">
                  <span></span>
                  <span></span>
                  <span></span>
                </div>
              </div>

              <div id="navbarExampleTransparentExample" class="navbar-menu">
                <div class="navbar-start">
                  <a class="navbar-item" href="/">
                    { "Home" }
                  </a>
                </div>

                <div class="navbar-end">
                  <div class="navbar-item">
                    <div class="field is-grouped">
                      <p class="control">
                        <a class="button is-primary" href="#" onclick={link.callback(|_| Msg::AddOne)}>
                          <span>{ "ログイン" }</span>
                        </a>
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </nav>
            <section class="section">
              <div class="container">
                <h1 class="title">
                  { "Hello World" }
                </h1>
                <p class="subtitle">
                  { "My first website with " }<strong>{ "Bluma" }</strong>
                </p>
              </div>
            </section>
          </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
