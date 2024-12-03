#![cfg(target_os = "windows")]
#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use nwg::NativeUi;
#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    layout: nwg::GridLayout,
    username: nwg::TextInput,
    password: nwg::TextInput,
    logout: nwg::Button,
    login: nwg::Button,
}
impl BasicApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
    fn login(&self) {
        let args = giwifi::Args {
            username: self.username.text(),
            password: self.password.text(),
            ..Default::default()
        };
        let client = reqwest::blocking::Client::builder()
            .user_agent(&args.ua)
            .build()
            .unwrap();
        match giwifi::login::main(&args, &client) {
            Ok(_) => {
                nwg::modal_info_message(&self.window, "登录成功", "登录成功");
            }
            Err(e) => {
                nwg::modal_error_message(&self.window, "登录失败", &format!("{:?}", e));
            }
        };
    }
    fn logout(&self) {
        let v = giwifi::Args {
            username: self.username.text(),
            password: self.password.text(),
            ..Default::default()
        };
        let client = reqwest::blocking::Client::builder()
            .user_agent(&v.ua)
            .build()
            .unwrap();
        match giwifi::logout::main(&v, &client) {
            Ok(_) => {
                nwg::modal_info_message(&self.window, "注销成功", "注销成功");
            }
            Err(e) => {
                nwg::modal_error_message(&self.window, "注销失败", &format!("{:?}", e));
            }
        };
    }
}
mod basic_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;

            let dpi = unsafe { nwg::dpi() };
            let [total_width, total_height] = [nwg::Monitor::width(), nwg::Monitor::height()];
            let w = total_width / dpi * 96;
            let h = total_height / dpi * 96;
            let x = (w - 300) / 2;
            let y = (h - 200) / 2;
            println!("{}x{}", x, y);

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((x, y))
                .title("Giwifi Toolkits")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .parent(&data.window)
                .placeholder_text(Some("用户名"))
                .build(&mut data.username)?;

            nwg::TextInput::builder()
                .parent(&data.window)
                .placeholder_text(Some("密码"))
                .build(&mut data.password)?;

            nwg::Button::builder()
                .text("注销")
                .parent(&data.window)
                .build(&mut data.logout)?;

            nwg::Button::builder()
                .text("登录")
                .parent(&data.window)
                .build(&mut data.login)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &evt_ui.login {
                                BasicApp::login(&evt_ui);
                            } else if &handle == &evt_ui.logout {
                                BasicApp::logout(&evt_ui);
                            }
                        }
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                BasicApp::exit(&evt_ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(2)
                .child_item(nwg::GridLayoutItem::new(&ui.username, 0, 0, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.password, 0, 1, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.logout, 0, 2, 1, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.login, 1, 2, 1, 1))
                .build(&ui.layout)?;

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
