use orbtk::api::*;
use orbtk::prelude::*;

use crate::click_point::ClickView;
use crate::count_up::CountView;

mod click_point;
mod count_up;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk Example")
                // 端末画面の左上から(100, 100）の位置に配置
                .position((100.0, 100.0))
                // リサイズはさせない
                .resizeable(false)
                .size(300.0, 300.0)
                // buildでEntityを登録
                // ちなみにCountViewのコメントアウトを除くと
                // 両方が上書きされるように表示される
                //.child(CountView::create().build(ctx))
                .child(ClickView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
