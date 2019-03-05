# RustのGUI用ライブラリ比較
RustでGUIを作成しようにも良さげなクレートでこれだというものがまだ決まっていないようなので良さげなクレートの簡単な比較をしていきます。
ここではそれぞれのライブラリでボタンを押すと数字が増える"カウントアップ"というGUIを作っていきます。
コードなどの確認は[こちら](https://github.com/ogata-k/GUI_cmp)を確認してください。

# azul
Azul(version ```0.1.0```)はRust```1.28```以降をサポートしているので、```rustc -vV```であなたのRustコンパイラのバージョンがサポートされているか確認してください。
## どんなクレートか
[azul](https://github.com/maps4print/azul)はMozillaのブラウザーのレンダリングエンジンを元にRustで書かれているIMGUI指向の(つまり"Immediate Mode GUI"と呼ばれるパラダイムを採用した)GUIフレームワークです。IMGUI指向を簡単にいうと"要求されたときに要求されたことだけを処理する"といった感じ(間違っているかも)になります。
AzulはHTMLのようなDOMスタイルを使いウィジェットの構造を表し、(Azul用の)CSSでレイアウトに対処するという形を取っています。
詳しくはAzulの[Tutorial(英語)](https://github.com/maps4print/azul/wiki)や[公式サイト](https://azul.rs/)を参照してみてください。
## 基本的なコード
``` style.css
#label{
  color: black;
  font-size: 60px;
}

#button{
  font-size: 10px;
  padding: 5px;
}
```

``` main.rs
extern crate azul;

use azul::{prelude::*, widgets::{label::Label, button::Button}};
use azul::window_state::WindowSize;
// レイアウトに依存するデータモデルの定義
struct DataModel {
  count_num: usize,
}

// レイアウトの実装
impl Layout for DataModel {
  // render関数
  fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
    // domでビルドするビルダーパターンのイメージでwidgetの作成
    let label = Label::new(format!("{}", self.count_num)).dom().with_id("label");
    // domにしてから関数を設定
    let button = Button::with_label("カウントアップ +1").dom().with_id("button")
      .with_callback(On::MouseUp, Callback(update_counter));

    // HTMLのような感じでレイアウトの部品となるDomを返す
    Dom::new(NodeType::Div)
      .with_child(label)
      .with_child(button)
  }
}


// appの情報とイベントの情報を受け取って計算したあとにスクリーンに状態を伝搬する関数
fn update_counter(app_state: &mut AppState<DataModel>, _event: &mut CallbackInfo<DataModel>) -> UpdateScreen {
  app_state.data.modify(|state| state.count_num += 1);
  // 再描画の必要が無いときはRedrawの代わりにDontRedrawを使う
  Redraw
}

fn main() {
  // GUIのルートの作成
  // 引数はレイアウトを決定する初期条件とログやエラー処理に関するデータ構造
  let app = App::new(DataModel {count_num: 0}, AppConfig::default());

  // Windowの設定
  let mut window_options = WindowCreateOptions::default();
  window_options.state.title = "カウントアップ".to_string();
  let mut window_size = WindowSize::default();
  window_size.dimensions = LogicalSize::new(400.0, 300.0);  // width * height
  window_options.state.size = window_size;

  // CSSの設定
  macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/style.css")) }
  let css = css::override_native(include_str!(CSS_PATH!())).expect(&format!("failed: override CSS by {}", CSS_PATH!()));

  app.run(Window::new(window_options, css).expect("failed: make window")).expect("failed: start running application");
}
```
## 所感
コンパイルは重い。そしてまだまだ発展途中ではある。また、ウィンドウのサイズやマウスの設定は構造体で与えてやる必要があるが、それ以外のレイアウトに関する情報は大体CSSで記述できる。このCSSで記述するというのはIMGUI指向のレイアウトを複雑にしにくいというのを解決してくれるように感じる。しかし個人的にうまく機能してくれなかった。（適用するための情報を求む）
Windowの細かい設定もしやすいのも特徴である。もちろんDefaultトレイトも実装されている。そのためデフォルトの一部を変えた設定を使うことも容易である。しかし設定できることが多い為、ものによっては設定するためのコードのネストが深くなってしまう。set〇〇のようなメソッドがほしいところ。
## 総評
まだまだ未熟な点が多く、ドキュメントもほとんど無いため実用には程遠く感じる。しかし、IMGUI指向のCSSでレイアウトが強化されたDOM型GUIライブラリは使いやすく感じるのでversion```1.0.0```を超えての安定バージョンを期待したいところ。
## ひとこと
まだまだ未熟だが、CSSでレイアウトを強化されたIMGUI指向のDOM型GUIライブラリ。

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
## 所感
他の言語のGTKのように親子関係からなる有向グラフで矢（有向辺）のラベルが保持の仕方（例えば水平型のスタック）という関係でコードを書いていくクレートであった。もちろんビルダーもあるのでファイルから読み込んで生成することもできる。
もしかしたら僕の検索能力が低いだけかもしれないが、文字サイズなどを改造しようとした途端に情報が少なくなる印象を受けた。[一応htmlのように書くことはできる](https://gtk-rs.org/docs/gtk/struct.Label.html#markup-styled-text)らしい。ただhtml風に書くと中身の取得時にタグの部分が邪魔になったり、タグを退けるために違う関数を呼び出す必要が出てくる。
## 総評
データと表示の分離がまだまだな印象が強いので、細かい設定をしだすと大変複雑になってきそうな印象を受けた。


