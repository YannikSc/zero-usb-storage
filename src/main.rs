use gtk::{prelude::*, Builder, Spinner};
use gtk::{Application, ApplicationWindow};
use std::sync::Arc;

#[derive(Clone)]
struct ApplicationWrap {
    gtk_app: Arc<Application>,
}

unsafe impl Send for ApplicationWrap {}
unsafe impl Sync for ApplicationWrap {}

impl ApplicationWrap {
    pub fn new(app: Application) -> Self {
        Self {
            gtk_app: Arc::new(app),
        }
    }
    pub fn gtk(&self) -> Arc<Application> {
        self.gtk_app.clone()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting");

    let app = ApplicationWrap::new(
        Application::builder()
            .application_id("me.schwieger.yannik.zero-usb-storage")
            .build(),
    );
    let gtk = app.gtk();

    gtk.connect_activate(on_activate);

    let thread_app = app.clone();

    let handle = std::thread::spawn(move || {
        let gtk = thread_app.gtk();
        gtk.run();
    });

    handle.join().expect("Unable to join thread");

    Ok(())
}

fn load_images() {}

fn on_activate(app: &Application) {
    let builder = Builder::from_string(include_str!("../ui/myui.glade"));
    let window: ApplicationWindow = builder
        .object("loading_window")
        .expect("Unable to find main_window");
    let spinner: Spinner = builder.object("spinner").expect("Unable to find spinner");
    spinner.start();
    window.set_application(Some(app));

    window.present();
}
