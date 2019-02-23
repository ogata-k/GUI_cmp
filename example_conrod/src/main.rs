#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

widget_ids!(
    struct Ids {
        canvas,
        title,
        text_box,
        button,
        result,
    });

fn fib(x: u64) -> u64 {
    match x {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    const TITLE: &'static str = "Fibonacci";
    let width = 300;
    let height = 100;

    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(width, height)
        .with_title(TITLE)
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

    /*
    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
*/
    // Generate the widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut text = "0".to_string();
    let mut answer = "0".to_string();

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&display) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(
                    _,
                    _,
                    Some(glium::glutin::VirtualKeyCode::Escape),
                ) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        set_widgets(ui.set_widgets(), ids, &mut text, &mut answer);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, text: &mut String, answer: &mut String) {
    widget::Canvas::new()
        .pad(0.0)
        .color(conrod::color::rgb(0.2, 0.35, 0.45))
        .set(ids.canvas, ui);

    let canvas_wh = ui.wh_of(ids.canvas).unwrap();

    // title
    widget::Text::new("Fibonacci Calculuator")
        .mid_top_with_margin_on(ids.canvas, 5.0)
        .font_size(20)
        .color(color::WHITE)
        .set(ids.title, ui);

    // textbox
    for event in widget::TextBox::new(text)
        .font_size(15)
        .w_h((canvas_wh[0] - 90.) / 2., 30.0)
        .mid_left_with_margin_on(ids.canvas, 30.0)
        .border(2.0)
        .border_color(color::BLUE)
        .color(color::WHITE)
        .set(ids.text_box, ui)
    {
        match event {
            widget::text_box::Event::Enter => println!("TextBox {:?}", text),
            widget::text_box::Event::Update(string) => *text = string,
        }
    }

    // button
    if widget::Button::new()
        .w_h((canvas_wh[0] - 90.) / 2., 30.0)
        .right_from(ids.text_box, 30.0)
        .rgb(0.4, 0.75, 0.6)
        .border(2.0)
        .label("calc!")
        .set(ids.button, ui)
        .was_clicked()
    {
        if let Ok(num) = text.parse::<u64>() {
            *answer = fib(num).to_string();
        } else {
            println!("invalid number");
        }
    }

    // result
    widget::Text::new(answer)
        .mid_bottom_with_margin_on(ids.canvas, 10.0)
        .font_size(20)
        .color(color::WHITE)
        .set(ids.result, ui);
}

struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
