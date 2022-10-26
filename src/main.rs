use std::{thread, time::Duration};

use fltk::{app::{App, channel}, window::Window, button::Button, prelude::{WidgetBase, WidgetExt, GroupExt, WindowExt}, misc::Progress};

#[derive(Clone, Copy)]
enum Message {

    StartProgress,
    FinishedProgress

}
fn main() {

    let (tx,rx) = channel::<Message>();
    
    let app = App::default();
    let mut wind = Window::default()
    .with_size(400, 200)
    .center_screen();

    let mut bt = Button::new(0, 10, 80, 30, "Start")
    .center_x(&wind);

    bt.set_callback(move |x|{

        tx.send(Message::StartProgress);

    });

    let progress = Progress::default().with_size(300, 40).center_of_parent();

    wind.end();
    wind.show();

    while app.wait() {

        match rx.recv() {
            Some(Message::StartProgress) => {

                bt.deactivate();

                let mut progress = progress.clone();

                thread::spawn(move ||{
                    
                    let mut cont = 1.1;
                    
                    while cont <= 100.0 {

                        progress.set_value(cont);

                        cont += 1.0;

                        thread::sleep(Duration::from_millis(20))

                    }

                    tx.send(Message::FinishedProgress)

                });
            },
            Some(Message::FinishedProgress) => {

                bt.activate();

            },
            _ => {}
        }

        wind.redraw();

    }

}