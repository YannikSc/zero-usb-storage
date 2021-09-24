use crate::image_loader::{Gadget, IconKind};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{
    prelude::*, Align, Box, Builder, EventBox, FlowBox, IconSize, Image, Label, Orientation,
};
use gtk::{Application, ApplicationWindow};
use std::sync::Arc;

use crate::image_loader::GadgetLoader;

#[derive(Clone)]
pub struct Gui {
    application: Arc<Application>,
}

pub trait ToGtkImage {
    fn to_image(&self) -> Image;
}

pub trait ToGadgetIcon {
    fn to_gadget_icon(&self) -> Box;
}

impl ToGtkImage for IconKind {
    fn to_image(&self) -> Image {
        let mut builder = Image::builder()
            .icon_name("drive-harddrive")
            .icon_size(IconSize::Dialog);

        match self {
            IconKind::File(file_name) => {
                if let Ok(image) = Pixbuf::from_file_at_scale(file_name, 48, 48, true) {
                    builder = builder.pixbuf(&image);
                }
            }
            IconKind::Resource(icon_name) => {
                builder = builder.icon_name(icon_name);
            }
        }

        builder.build()
    }
}

impl ToGtkImage for Gadget {
    fn to_image(&self) -> Image {
        match self {
            Gadget::MassStorage(image_info) => image_info.icon.to_image(),
            Gadget::Serial => self.icon_image("connect"),
            Gadget::Ethernet => self.icon_image("connect"),
        }
    }
}

impl ToGadgetIcon for Gadget {
    fn to_gadget_icon(&self) -> Box {
        let name_label = Label::builder()
            .label(match self {
                Gadget::MassStorage(image) => image.name.as_str(),
                Gadget::Serial => "Serial",
                Gadget::Ethernet => "Ethernet",
            })
            .build();
        let image_wrap = Box::builder()
            .height_request(50)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();
        image_wrap.add(&self.to_image());

        let container = Box::builder().orientation(Orientation::Vertical).build();
        container.add(&image_wrap);
        container.add(&name_label);

        if let Self::MassStorage(image) = self {
            let path_label = Label::builder().label(image.path.as_str()).build();

            container.add(&path_label);
        }

        container
    }
}

impl Gadget {
    fn icon_image(&self, icon_name: &str) -> Image {
        Image::builder()
            .icon_name(icon_name)
            .icon_size(IconSize::Dialog)
            .build()
    }
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

        self.load_gadgets(app, &builder);
    }

    fn load_gadgets(&self, app: &Application, builder: &Builder) {
        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Unable to find main_window");
        window.set_application(Some(app));
        window.present();

        self.print_gadgets(app, builder, GadgetLoader::new().load());
    }

    fn print_gadgets(&self, _app: &Application, builder: &Builder, gadgets: Vec<Gadget>) {
        let flow_box: FlowBox = builder
            .object("item_list")
            .expect("Could not find item_list");

        for gadget in gadgets {
            let event_box = EventBox::builder().build();

            let gadget_clone = gadget.clone();
            let self_clone = self.clone();
            event_box.connect_button_press_event(move |_, _| {
                self_clone.on_pressed_image(gadget_clone.clone());

                Inhibit(true)
            });

            event_box.add(&gadget.to_gadget_icon());
            flow_box.add(&event_box);
        }

        flow_box.show_all();
    }

    fn on_pressed_image(&self, gadget: Gadget) {
        if let Ok(execution_script) = std::env::var("MODPROBE_SCRIPT") {
            let arguments = gadget.module_arguments();
            let module = gadget.module_name();

            if let Ok(output) = std::process::Command::new(&execution_script)
                .arg(&module)
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
                    "Could not run execution command: {} {} {:?}",
                    &execution_script, &module, &arguments
                );
            }
        } else {
            eprintln!("Missing MODPROBE_SCRIPT");
        }
    }
}
