use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

mod bindings;

enum Msg {
    LogIn,
    AuthConfigFetched(Result<AuthConfig, reqwasm::Error>),
}

struct Model {
    value: i64,
    auth_config: Option<AuthConfig>,
}

#[derive(Deserialize)]
struct AuthConfig {
    domain: String,
    client_id: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            link.send_message(Msg::AuthConfigFetched(
                async { Request::get("/auth_config.json").send().await?.json().await }.await,
            ));
        });
        Self {
            value: 0,
            auth_config: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LogIn => {
                let payload = bindings::get_payload();
                log::info!("js_value: {}", payload);
                self.value += 1;
                true
            }
            Msg::AuthConfigFetched(Ok(auth_config)) => {
                log::info!("domain {}", auth_config.domain);
                log::info!("client_id {}", auth_config.client_id);
                true
            }
            Msg::AuthConfigFetched(Err(fetch_error)) => {
                log::info!("error occured {}", fetch_error);
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
                        <a class="button is-primary" href="#" onclick={link.callback(|_| Msg::LogIn)}>
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
