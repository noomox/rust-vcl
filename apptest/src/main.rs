#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(dead_code)]

// #[cfg(target_os = "windows")]
#[macro_use]
extern crate rust_vcl;

use rust_vcl::fns::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;

// 按钮1单击事件
fn onBtnClick(sender: usize) {
    ShowMessage("Hello, Rust! 你好，世界！");
    let btn = TButton::As(sender);
    println!("caption: {:?}", btn.Caption());
}

fn onBtn2Click(_sender: usize) {
    //TStrings是抽像类，当参数类型为TStrings要用TStringList传入，非TComponent实现了drop方法，所以Free不是必须的
    let list = TStringList::new();
    list.Add("Item1");
    list.Add("Item2");
    list.Add("Item3");
    let idx = InputCombo(
        "caption",
        "Prompt                          ", // aPrompt参数决定显示的窗口宽度
        &list,
    );
    println!("select index: {}", idx);
}

fn onBtn3Click(_sender: usize) {
    // 集合测试
    // 加法
    let val1 = Include!(0, 1, 2, 3); // result=14
    println!("val1={}", val1);
    // 减法
    let val2 = Exclude!(val1, 2); // result=10
    println!("val2={}", val2);

    println!("2 in set={}", InSet!(val2, 2));
    println!("3 in set={}", InSet!(val2, 3));

    ShowMessage("消息");
    if MessageDlg(
        "消息",
        TMsgDlgType::mtConfirmation,
        Include!(0, TMsgDlgBtn::mbYes, TMsgDlgBtn::mbNo),
        0,
    ) == mrYes
    {
        ShowMessage("你点击了“是")
    }
    // windows consts
    // if Application.MessageBox("消息", "标题", MB_OKCANCEL | MB_ICONINFORMATION) == idOK  {
    //     ShowMessage("你点击了“是")
    // }
}

fn onDropFile(_sender: usize, fileNames: usize, len: isize) {
    println!("{}, {}, {}", _sender, fileNames, len);
    for i in 0..len {
        let fileName = GetFPStringArrayMember(fileNames, i);
        println!("{}: {:?}", i, &fileName);
    }
}

fn onFormMouseMove(_sender: usize, shift: TShiftState, _x: i32, _y: i32) {
    if InSet!(shift, TShiftStateEnum::ssCtrl) {
        println!("ctrl");
    }
    // let form = TForm::As(_sender);
    // let pos = Mouse.CursorPos();
    // println!("x={}, y={}, sx={}, sy={}", _x, _y, pos.x, pos.y);
}

fn test() {
    let guid = CreateGUID();
    println!("{}-{}-{}-{:?}", guid.d1, guid.d2, guid.d3, guid.d4);
    println!("{:?}", GUIDToString(&guid));
    println!("{:?}", LibAbout());

    // let abc = TGridRect::Empty();
}

fn main() {
    test();

    Application.SetMainFormOnTaskBar(true);
    Application.SetTitle("LCL App");
    Application.Initialize();
    Application.Icon().LoadFromFile("applogo.ico");

    let form = Application.CreateForm();
    form.SetCaption("你好，Rust！ - Hello Rust!");
    form.SetPosition(TPosition::poScreenCenter);
    form.SetAllowDropFiles(true);
    form.SetOnDropFiles(onDropFile);
    form.SetOnMouseMove(onFormMouseMove);
    // form.SetOnClick(onBtnClick);

    // 测试自动drop
    // let ico = TIcon::new();
    //println!("{:?}", ico.ClassName());

    let btn = TButton::new(&form);
    btn.SetParent(&form);
    btn.SetLeft(10);
    btn.SetTop(50);
    btn.SetCaption("button1");
    btn.SetOnClick(onBtnClick);

    let btn2 = TButton::new(&form);
    btn2.SetParent(&form);
    btn2.SetLeft(10);
    btn2.SetTop(btn.Top() + btn.Height() + 10);
    btn2.SetWidth(120);
    btn2.SetCaption("InputCombo");
    btn2.SetOnClick(onBtn2Click);

    let btn3 = TButton::new(&form);
    btn3.SetParent(&form);
    btn3.SetLeft(10);
    btn3.SetTop(btn2.Top() + btn2.Height() + 10);
    btn3.SetCaption("MsgBox");
    btn3.SetOnClick(onBtn3Click);

    Application.Run();
}
