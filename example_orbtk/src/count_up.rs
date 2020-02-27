use std::cell::Cell;

// 他のモジュールも読み込んでいるのでこれだけで十分
use orbtk::prelude::*;

// 受け付ける処理
#[derive(Debug, Copy, Clone)]
enum CountViewAction {
    Increment,
}

// 状態を格納するための状態構造体
#[derive(Default)]
pub struct CountViewState {
    // 内部可変のためにCellで包む
    action: Cell<Option<CountViewAction>>,
    count: Cell<usize>,
}

impl CountViewState {
    // アクションが発火したことを通知
    fn set_action(&self, action: impl Into<Option<CountViewAction>>) {
        self.action.set(action.into());
    }
}

impl State for CountViewState {
    // 画面が更新されるたびに呼ばれるので適切に終了条件を指定してやる必要がある
    fn update(&self, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                CountViewAction::Increment => {
                    let result = self.count.get() + 1;
                    self.count.set(result);
                    ctx.child("text-block")
                        // 通常の文字列は対応していないので利用できる形に変換する
                        .set("text", String16::from(result.to_string()));
                }
            }
            // 通知イベントを削除
            self.action.set(None);
        }
    }
}

widget!(CountView<CountViewState> {});

impl Template for CountView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("CountView").child(
            Stack::create()
                .vertical_alignment("center")
                .child(
                    TextBlock::create()
                        .margin((8.0, 8.0, 8.0, 8.0))
                        // もしかしたら方法はあるかもしれないが日本語は対応していない
                        .text("Counter!!")
                        .horizontal_alignment("center")
                        // selectorのうちidをセット
                        .selector(Selector::new().id("text-block"))
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .text("count + 1")
                        .horizontal_alignment("center")
                        // クロージャーの引数はPointなので今回はアクション通知に必要ない
                        .on_click(move |_| {
                            // https://blog1.mammb.com/entry/2019/12/16/090000 のように
                            // 新しいとクロージャーの引数としてstateがわたってくるようになってる。
                            state.set_action(CountViewAction::Increment);
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
