#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

// 使用するid一覧
widget_ids!(
    struct Ids {
        canvas,
        num_lbl,
        button,
    });

fn main() {
  // 設定値
    const TITLE: &'static str = "カウントアップ";
    let width = 400;
    let height = 300;

    // windowの作成
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title(TITLE)
        .build_glium()  // windowの構築
        .unwrap();

    // Uiの作成
    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

    // Uiで使うFontをassets以下のファイルからfont::Mapに追加
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // idを管理するための管理者の作成
    let ids = &mut Ids::new(ui.widget_id_generator());

    // gliumで描画するためのrendererの準備
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The widgetとimageを結びつけて管理するmapping
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();


    let mut num = "0".to_string();

    let mut event_loop = EventLoop::new();
    // windowのイベントループ
    'main: loop {
        for event in event_loop.next(&display) {
            // windowのイベントのハンドラーをuiにセット
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                // Escapeはwindowの削除用に
                glium::glutin::Event::KeyboardInput(
                    _,
                    _,
                    Some(glium::glutin::VirtualKeyCode::Escape),
                ) |
                glium::glutin::Event::Closed => break 'main,
                _ => {}
            }
        }


        set_widgets(ui.set_widgets(), ids, &mut num);

        // Uiの描画とその表示
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// 配置するwidgetの配置方法の指定関数
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, num: &mut String) {
  // 背景(canvas)
    widget::Canvas::new()
        .pad(0.0)
        .color(conrod::color::rgb(0.2, 0.35, 0.45))
        .set(ids.canvas, ui);

    // canvasのidを使い指定することでuiからcanvasの横と縦の配列を取得
    let canvas_wh = ui.wh_of(ids.canvas).unwrap();


    // 数値の表示
    widget::Text::new(num)
        .mid_top_with_margin_on(ids.canvas, 10.0)
        .font_size(20)  // フォントのサイズを指定！！
        .color(color::WHITE)  // 色の指定も簡単にできる
        .set(ids.num_lbl, ui);

    // カウントボタン
    if widget::Button::new()
        .w_h((canvas_wh[0] - 90.) / 2., 30.0)  // 幅
        .left_from(ids.num_lbl, 10.0)// 位置
        .rgb(0.4, 0.75, 0.6)  // 色
        .border(2.0)  // 境界
        .label("count +1")
        .set(ids.button, ui)
        .was_clicked()
    {  // if式の実行部分
        if let Ok(count) = num.parse::<u32>() {
            *num = (count+1).to_string();
        } else {
            println!("invalid number");
        }
    }

}

// イベントの管理用構造体
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

    /// すべての更新対象となるイベントの為の順次取得用関数
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // 60FPSより早くならないようにするために一つ前の更新対象から少なくても16ms待つことにしておく。
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);  // 前回の更新時と今の時間差の取得
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // イベント全体の取得
        let mut events = Vec::new();
        events.extend(display.poll_events());  // displayにおけるイベントの取得

        // displayで更新があればUiでのイベント更新は次に持ち越し
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        // イベントの更新の後処理
        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    // Uiで他のイベントの更新があるかないかを要求することをeventのループでは確認しておくこと

    // これはいくつかのUiを描画する最初のタイミングや更新を要求するタイミングで使われる。
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
