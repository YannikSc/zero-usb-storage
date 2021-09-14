use gui::Gui;

mod gui;
mod image_loader;

fn main() {
    Gui::new().build_gtk();
}
