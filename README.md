# RustのGUI用ライブラリ比較
RustでGUIを作成しようにも良さげなクレートでこれだというものがまだ決まっていないようなので良さげなクレートの簡単な比較をしていきます。
ここではそれぞれのライブラリでボタンを押すと数字が増える"カウントアップ"というGUIを作っていきます。
コードなどの確認は[こちら](https://github.com/ogata-k/GUI_cmp)を確認してください。

# conrod
今回は[こちら](http://mmi.hatenablog.com/entry/2017/07/09/234945)を参考にしてそこに[挙げられている例](https://github.com/mmisono/conrod-examples/tree/master/fibonacci)の指定の通りのバージョン```0.53.0```を使用しています。

## どんなクレートか
## 基本的なコード
## 所感
assetsからフォントを引っ張ってきたりして、何かしらのフォントを指定してやらないと動いても文字が見えないのは少し面倒に感じます。
## 総評


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


