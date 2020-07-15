#![windows_subsystem = "windows"]

// #[cfg(target_os = "windows")]

use rust_vcl::enums::*;
use rust_vcl::fns::*;

use rust_vcl::vcl::*;

// 按钮1单击事件
fn on_btn_click(_sender: usize) {
    ShowMessage("Hello, Rust! 你好，世界！");
}

fn on_drop_file_event(_sender: usize, file_names: usize, len: isize) {
    println!("{}, {}, {}", _sender, file_names, len);

    for i in 0..len {
        let file_name = GetStringArrOf(file_names, i);
        println!("{}: {:?}", i, &file_name);
    }
}

fn main() {
    // 乱写的，也不知道是不是这样写
    // 这里因为不会写，所以就这样弄下

    Application.SetMainFormOnTaskBar(true);
    Application.SetTitle("LCL App");
    Application.Initialize();
    Application.Icon().LoadFromFile("applogo.ico");

    let form = Application.CreateForm();
    form.SetCaption("你好，Rust！ - Hello Rust!");
    form.SetPosition(TPosition::poScreenCenter);
    form.SetAllowDropFiles(true);
    form.SetOnDropFiles(on_drop_file_event);
    // form.SetOnClick(on_btn_click);

    let btn = TButton::new(&form);
    btn.SetParent(&form);
    btn.SetLeft(10);
    btn.SetTop(50);
    btn.SetCaption("button1");
    btn.SetOnClick(on_btn_click);

    Application.Run();
}
