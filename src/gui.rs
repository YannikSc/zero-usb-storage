use crate::image_loader::{IconKind, ImageInfo};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{
    prelude::*, Align, Box, Builder, EventBox, FlowBox, IconSize, Image, Label, Orientation,
};
use gtk::{Application, ApplicationWindow};
use std::sync::Arc;

use crate::image_loader::ImageLoader;

#[derive(Clone)]
pub struct Gui {
    application: Arc<Application>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            application: Arc::new(
                Application::builder()
                    .application_id("me.schwieger.yannik.zero-usb-storage")
                    .build(),
            ),
        }
    }

    pub fn build_gtk(&self) {
        let application = self.application.clone();
        let cloned_self = self.clone();
        application.connect_activate(move |app| cloned_self.on_activate(app));
        application.run();
    }

    fn on_activate(&self, app: &Application) {
        let builder = Builder::from_string(include_str!("../ui/myui.glade"));

        self.load_images(app, &builder);
    }

    fn load_images(&self, app: &Application, builder: &Builder) {
        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Unable to find main_window");
        window.set_application(Some(app));
        window.present();

        self.print_images(app, builder, ImageLoader::new().load_images());
    }

    fn print_images(&self, _app: &Application, builder: &Builder, images: Vec<ImageInfo>) {
        let flow_box: FlowBox = builder
            .object("item_list")
            .expect("Could not find item_list");

        for image in images {
            let mut image_builder = Image::builder()
                .icon_name("drive-harddrive")
                .icon_size(IconSize::Dialog);

            match &image.icon {
                IconKind::File(file_name) => {
                    if let Ok(image) = Pixbuf::from_file_at_scale(file_name, 48, 48, true) {
                        image_builder = image_builder.pixbuf(&image);
                    }
                }
                IconKind::Resource(icon_name) => {
                    image_builder = image_builder.icon_name(icon_name);
                }
            }

            let name_label = Label::builder().label(image.name.as_str()).build();
            let path_label = Label::builder().label(image.path.as_str()).build();
            let image_wrap = Box::builder()
                .height_request(50)
                .halign(Align::Center)
                .valign(Align::Center)
                .build();
            let event_box = EventBox::builder().build();
            let container = Box::builder().orientation(Orientation::Vertical).build();
            image_wrap.add(&image_builder.build());
            container.add(&image_wrap);
            container.add(&name_label);
            container.add(&path_label);

            let image_clone = image.clone();
            let self_clone = self.clone();
            event_box.connect_button_press_event(move |_, _| {
                self_clone.on_pressed_image(image_clone.clone());

                Inhibit(true)
            });

            event_box.add(&container);
            flow_box.add(&event_box);
        }

        flow_box.show_all();
    }

    fn on_pressed_image(&self, image: ImageInfo) {
        let mut arguments = vec![format!("file={}", &image.path)];

        if image.as_cdrom.unwrap_or(false) {
            arguments.push(String::from("cdrom=y"));
        }

        arguments.push(format!(
            "ro={}",
            if image.read_only.unwrap_or(false) {
                "1"
            } else {
                "0"
            }
        ));

        if let Ok(execution_script) = std::env::var("MODPROBE_SCRIPT") {
            if let Ok(output) = std::process::Command::new(&execution_script)
                .args(&arguments)
                .output()
            {
                if output.status.success() {
                    println!("Mounted image");
                } else {
                    eprintln!(
                        "Could not mount image: \n\nError:\n {}\n\nOutput: {}",
                        String::from_utf8(output.stderr)
                            .unwrap_or_else(|_| String::from("Binary Output")),
                        String::from_utf8(output.stdout)
                            .unwrap_or_else(|_| String::from("Binary Output"))
                    );
                }
            } else {
                eprintln!(
                    "Could not run execution command: {} {:?}",
                    &execution_script, &arguments
                );
            }
        } else {
            eprintln!("Missing MODPROBE_SCRIPT");
        }
    }
}
