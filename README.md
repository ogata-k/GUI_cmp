# RustのGUI用ライブラリ比較
RustでGUIを作成しようにも良さげなクレートでこれだというものがまだ決まっていないようなので良さげなクレートの簡単な比較をしていきます。
ここではそれぞれのライブラリでボタンを押すと数字が増える"カウントアップ"というGUIを作っていきます。
コードなどの確認は[こちら](https://github.com/ogata-k/GUI_cmp)を確認してください。

# gtk
## どんなクレートか
[gtk](https://github.com/gtk-rs/gtk)はGTK+ 3とCairo、GtkSourceView、そしてGLibの互換性のあるライブラリ（つまり[cairo-rs](https://crates.io/crates/cairo-rs), [gdk](https://crates.io/crates/gdk), [gdk-pixbuf](https://crates.io/crates/gdk-pixbuf),[gio](https://crates.io/crates/gio), [glib](https://crates.io/crates/glib), [gtk](https://crates.io/crates/gtk), [pango](https://crates.io/crates/pango), [sourceview](https://crates.io/crates/sourceview)）を結びつけるためのクレートです。
このgtkクレートはOSにGTK+とGLibとCairoをインストールされていることを要求するので注意が必要です。
## コード
% TODO 作成してからここに貼り付ける。
