use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

mod bindings;
use wasm_bindgen::prelude::*;

enum Msg {
    AuthConfigFetched(Result<AuthConfig, reqwasm::Error>),
    AuthInitialized(Result<JsValue, JsValue>),
    SignUp,
    LogIn,
    LogOut,
    RedirectingToSignUp(Result<(), JsValue>),
    RedirectingToLogIn(Result<(), JsValue>),
}

struct Model {
    value: i64,
    auth_config: Option<AuthConfig>,
    user: Option<User>,
}

#[derive(Deserialize)]
struct AuthConfig {
    domain: String,
    client_id: String,
}

#[derive(Deserialize, Debug)]
struct User {
    nickname: String,
    name: String,
    picture: String,
    updated_at: String,
    email: String,
    email_verified: bool,
    sub: String,
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
            user: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::AuthConfigFetched(Ok(auth_config)) => {
                let domain = auth_config.domain.clone();
                let client_id = auth_config.client_id.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    link.send_message(Msg::AuthInitialized(init_auth(domain, client_id).await));
                });

                log::info!("domain {}", auth_config.domain);
                log::info!("client_id {}", auth_config.client_id);
                self.auth_config = Some(auth_config);
                true
            }
            Msg::AuthConfigFetched(Err(fetch_error)) => {
                log::info!("error occured {}", fetch_error);
                true
            }
            Msg::AuthInitialized(Ok(user)) => {
                if !user.is_undefined() {
                    match serde_wasm_bindgen::from_value(user) {
                        Ok(user) => {
                            log::info!("user: {:?}", user);
                            self.user = Some(user);
                        }
                        Err(error) => {
                            log::info!("User deserialization failed!, {}", error);
                        }
                    }
                }
                true
            }
            Msg::AuthInitialized(Err(error)) => {
                log::info!("error occured.");
                true
            }
            Msg::SignUp => {
                wasm_bindgen_futures::spawn_local(async move {
                    link.send_message(Msg::RedirectingToSignUp(redirect_to_sign_up().await));
                });
                true
            }
            Msg::LogIn => {
                wasm_bindgen_futures::spawn_local(async move {
                    link.send_message(Msg::RedirectingToLogIn(redirect_to_log_in().await));
                });
                true
            }
            Msg::RedirectingToSignUp(result) => {
                if let Err(error) = result {
                    log::info!("Redirect to sign up failed! {:?}", error);
                }
                true
            }
            Msg::RedirectingToLogIn(result) => {
                if let Err(error) = result {
                    log::info!("Redirect to log in failed! {:?}", error);
                }
                true
            }
            Msg::LogOut => {
                if let Err(error) = logout() {
                    log::info!("Cannot log out! {:?}", error);
                } else {
                    self.user = None;
                }
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
                        <a class="button is-primary mr-2" href="#" onclick={link.callback(|_| Msg::LogIn)}>
                          <span>{ "ログイン" }</span>
                        </a>
                        <a class="button is-primary" href="#" onclick={link.callback(|_| Msg::SignUp)}>
                          <span>{ "サインアップ" }</span>
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn init_auth(domain: String, client_id: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn redirect_to_sign_up() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn redirect_to_log_in() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn logout() -> Result<(), JsValue>;
}
