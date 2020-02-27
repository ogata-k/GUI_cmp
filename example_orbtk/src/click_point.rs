use std::cell::Cell;

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum ClickViewAction {
    // クリックされた箇所を通知するために包んでおく
    Click(Point),
}

#[derive(Default)]
pub struct ClickViewState {
    // 内部可変で扱うので各自状態で保持しておきたいものをCellで包む
    action: Cell<Option<ClickViewAction>>,
}

impl ClickViewState {
    // アクションの状態を更新するためのヘルパーメソッド
    fn set_action(&self, action: impl Into<Option<ClickViewAction>>) {
        self.action.set(action.into());
    }
}

impl State for ClickViewState {
    // 画面が更新されるたびに呼ばれるので適切に終了条件を指定してやる必要がある
    fn update(&self, context: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                ClickViewAction::Click(p) => {
                    context
                        .child("click-text")
                        // 通常の文字列は対応していないので利用できる形に変換する
                        .set("text", String16::from(format!("({}, {})", p.x, p.y)));
                }
            }
        }
        // 通知イベントを削除
        self.action.set(None);
    }
}

// クリックイベントは以下のよう指定するだけでは作れず、be
// buttonの実装の
// https://github.com/redox-os/orbtk/blob/develop/crates/widgets/src/button.rs
// を見る限り方法はあるはず
widget!(ClickView<ClickViewState>: MouseHandler{});

impl Template for ClickView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("ClickView")
            .child(
                TextBlock::create()
                    .text("Click!")
                    .horizontal_alignment("center")
                    .vertical_alignment("center")
                    // selectorのうちidをセット
                    .selector(Selector::new().id("click-text"))
                    // buildでEntityを登録
                    .build(context),
            )
            // マウス動作確認用(動いていないことが確認できる)
            .on_click(move |p| {
                println!("on click");
                false
            })
            .on_mouse_up(move |p| {
                println!("on mouse up");
                state.set_action(ClickViewAction::Click(p));
                // FIXME ここの戻り値の意味が分からん
                false
            })
    }
}
