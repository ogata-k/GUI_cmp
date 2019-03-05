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
