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

