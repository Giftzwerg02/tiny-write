use gtk::{glib, Application, PropagationPhase, Button, GestureDrag};
use gtk::{
    prelude::*, ApplicationWindow, EventController, EventControllerLegacy, Gesture, GestureStylus,
};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let tablet = GestureStylus::builder()
        .stylus_only(false)
        .build();




    tablet.connect_ev.

    tablet.connect_up(|t, x, y| {
        dbg!(t);
        dbg!(t.device_tool());
        dbg!(t.backlog());
    });


    tablet.connect_down(|t, x, y| {
        println!("{} {}", x, y);
    });


    let window = ApplicationWindow::builder()
        .application(app)
        .title("amogus")
        .build();

    window.add_controller(tablet);

    window.present();
}
