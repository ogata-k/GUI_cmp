# RustのGUI用ライブラリ比較
RustでGUIを作成しようにも良さげなクレートでこれだというものがまだ決まっていないようなので良さげなクレートの簡単な比較をしていきます。
ここではそれぞれのライブラリでボタンを押すと数字が増える"カウントアップ"というGUIを作っていきます。
コードなどの確認は[こちら](https://github.com/ogata-k/GUI_cmp)を確認してください。

---

# conrod
今回は[こちら](http://mmi.hatenablog.com/entry/2017/07/09/234945)を参考にしてそこに[挙げられている例](https://github.com/mmisono/conrod-examples/tree/master/fibonacci)の指定の通りのバージョン```0.53.0```を使用している。
## どんなクレートか
conrodは[piston](https://www.piston.rs/)というゲームエンジンを開発しているところが作成しているRust純正のGUIライブラリで、Widgetの構成や管理、Widgetに対するイベントの伝播などの基本的な機能を担当できます。しかし、実際の描画やOSからのイベントの受取はconrodはできず、conrodに用意されている描画用の[glium](https://github.com/glium/glium)(OpenGL)、イベントの管理用の[winit](https://github.com/tomaka/winit)のバックエンドが用意されているので、そちらを使うことになる。
マルチプラットフォームで動作する。
[こちら](https://github.com/PistonDevelopers/conrod/tree/master/backends/conrod_glium/examples)に例がいくつか載っているので参考に成ると思う。

## 基本的なコード
```
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
        .middle_of(ids.canvas)
        .font_size(140)  // フォントのサイズを指定！！
        .color(color::WHITE)  // 色の指定も簡単にできる
        .set(ids.num_lbl, ui);

    // カウントボタン
    if widget::Button::new()
        .w_h(canvas_wh[0] - 10.0, 40.0)  // 幅
        .mid_bottom_with_margin_on(ids.canvas, 5.0)// 位置
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
```
![conrod完成品](/pictures/conrod.png)
## 所感
assetsからフォントを引っ張ってきたりして、何かしらのフォントを指定してやらないと動いても文字が見えないのは少し面倒に感じます。そして日本語には対応していないというめんどくささも。さらに特徴であるIDによる管理も大変めんどくさい。簡単なGUIでも面倒くさいのだからコードが複雑になればなるほど管理しきれなくなってくると思う。このIDというのはただ単に面倒くさいものではなくて、Widgetの変数名のような扱いが出来るのでWidgetの管理、Widgetの特定の容易化出来るようにしている。
ただフォントをわざわざ用意するためか、conrodは比較的色とかサイズとかのレイアウト設定がやりやすいが、配置の設定はgtkのほうがまだやりやすく感じる。
## 総評
まだまだチュートリアルなどの情報が少なく調べにくいが、ラベルのサイズや色の変更が簡単で細かく装飾したい人には良さそうなクレートだと思われる。しかし、Idでの管理の面倒臭さやレイアウト、データと処理の分離がまだまだだと思われる。
## ひとこと
装飾は強いがレイアウトが弱い。

---

# gtk
今回使用するバージョンは```0.5.0```です。
## どんなクレートか
[gtk](https://github.com/gtk-rs/gtk)はGTK+ 3とCairo、GtkSourceView、そしてGLibの互換性のあるライブラリ（つまり[cairo-rs](https://crates.io/crates/cairo-rs), [gdk](https://crates.io/crates/gdk), [gdk-pixbuf](https://crates.io/crates/gdk-pixbuf),[gio](https://crates.io/crates/gio), [glib](https://crates.io/crates/glib), [gtk](https://crates.io/crates/gtk), [pango](https://crates.io/crates/pango), [sourceview](https://crates.io/crates/sourceview)）を結びつけるためのクレートです。
このgtkクレートはOSに[GTK+とGLibとCairoをインストール](https://gtk-rs.org/docs-src/requirements.html)されていることを要求するので注意が必要です。
## 基本的なコード
```
extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

fn main(){
  if gtk::init().is_err() {
    println!("Faild to initialize GTK");
    return;
  }

  // windowの作成
  let window = Window::new(WindowType::Toplevel);
  window.set_title("カウントアップ");
  window.set_default_size(400, 300);  // 横×縦

  // 各widgetの作成
  let label = Label::new("0");
  let button = Button::new_with_label("+1");
  
  // windowへの配置
  let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
  // child: &P, expand: bool, fill: bool, padding: u32
  vbox.pack_start(&label, true, true, 3); 
  vbox.pack_start(&button, false, true, 2);
  window.add(&vbox);
  
  // 初期表示
  window.show_all();

  // eventの設定
  window.connect_delete_event(|_, _| {
    gtk::main_quit();
    Inhibit(false)
  });

  let label_c = label.clone();
  button.connect_clicked(move |_| {
    let old_num: u16 = label_c.get_text().unwrap().to_string().parse::<u16>().unwrap();
    label.set_text(&(old_num + 1).to_string());
    println!("カウント+1");
  });

  // GUIの実行
  gtk::main();
}
```
![gtk-rs完成品](/pictures/gtk_rs.png)
## 所感
他の言語のGTKのように親子関係からなる有向グラフで矢（有向辺）のラベルが保持の仕方（例えば水平型のスタック）という関係でコードを書いていくクレートであった。もちろんビルダーもあるのでファイルから読み込んで生成することもできる。
もしかしたら僕の検索能力が低いだけかもしれないが、文字サイズなどを改造しようとした途端に情報が少なくなる印象を受けた。[一応htmlのように書くことはできる](https://gtk-rs.org/docs/gtk/struct.Label.html#markup-styled-text)らしい。ただhtml風に書くと中身の取得時にタグの部分が邪魔になったり、タグを退けるために違う関数を呼び出す必要が出てくる。
## 総評
データと表示の分離がまだまだな印象が強いので、細かい設定をしだすと大変複雑になってきそうな印象を受けた。
## ひとこと
レイアウトは強いが装飾が弱い。

