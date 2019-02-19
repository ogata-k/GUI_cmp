# RustのGUI用ライブラリ比較
RustでGUIを作成しようにも良さげなクレートでこれだというものがまだ決まっていないようなので良さげなクレートの簡単な比較をしていきます。
ここではそれぞれのライブラリでボタンを押すと数字が増える"カウントアップ"というGUIを作っていきます。
コードなどの確認は[こちら](https://github.com/ogata-k/GUI_cmp)を確認してください。

# gtk
今回使用するバージョンは```0.5.0```です。
## どんなクレートか
[gtk](https://github.com/gtk-rs/gtk)はGTK+ 3とCairo、GtkSourceView、そしてGLibの互換性のあるライブラリ（つまり[cairo-rs](https://crates.io/crates/cairo-rs), [gdk](https://crates.io/crates/gdk), [gdk-pixbuf](https://crates.io/crates/gdk-pixbuf),[gio](https://crates.io/crates/gio), [glib](https://crates.io/crates/glib), [gtk](https://crates.io/crates/gtk), [pango](https://crates.io/crates/pango), [sourceview](https://crates.io/crates/sourceview)）を結びつけるためのクレートです。
このgtkクレートはOSに[GTK+とGLibとCairoをインストール](https://gtk-rs.org/docs-src/requirements.html)されていることを要求するので注意が必要です。
## コード
% TODO 作成してからここに貼り付ける。
## 所感
他の言語のGTKのように親子関係からなる有向グラフで矢（有向辺）のラベルが保持の仕方（例えば水平型のスタック）という関係でコードを書いていくクレートであった。もちろんビルダーもあるのでファイルから読み込んで生成することもできる。
もしかしたら僕の検索能力が低いだけかもしれないが、文字サイズなどを改造しようとした途端に情報が少なくなる印象を受けた。[一応htmlのように書くことはできる](https://gtk-rs.org/docs/gtk/struct.Label.html#markup-styled-text)らしい。ただhtml風に書くと中身の取得時にタグの部分が邪魔になったり、タグを退けるために違う関数を呼び出す必要が出てくる。
## 総評
データと表示の分離がまだまだな印象が強いので、細かい設定をしだすと大変複雑になってきそうな印象を受けた。


