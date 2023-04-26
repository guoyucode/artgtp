use std::{
    thread,
};

slint::slint! {
    import { SpinBox, Button, CheckBox, Slider, LineEdit, ScrollView, ListView,
        HorizontalBox, VerticalBox, GridBox, StandardButton } from "std-widgets.slint";
    import { HorizontalBox, VerticalBox, ListView, StandardListView, GroupBox } from "std-widgets.slint";

    export component HelloWorld inherits Window {
        title: "ArtGPT";
        min-width: 500px;
        min-height: 600px;

        // preferred-width: 700px;
        // preferred-height: 800px;

        callback popup_confirmed;
        callback show_confirm_popup;
        show_confirm_popup => { confirm_popup.show(); }

        HorizontalBox {   
            vertical-stretch: 1;
            GroupBox {  
                title: "会话列表";
                width: 20%;
                ListView {  
                    width: 95%;
                    vertical-stretch: 0;
                    for i in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] : HorizontalBox {
                    Image {
                            width: 15px;
                    }
                    Text {
                            text: "Item " + i;
                    }
                    }
                }
            }

            GroupBox {  
                title: "聊天窗口";
                vertical-stretch: 0;

                // 水平布局
                // HorizontalLayout
                VerticalLayout{
                    alignment: center;

                    StandardListView {
                        width: 90%;
                        height:  root.height - 180px;
                        model: [
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                            {text: "Lorem"}, {text: "ipsum"},{text: "dolor"},{text: "sit"},{text: "amet"},{text: "consetetur"},
                        ];
                    }

                    HorizontalLayout {
                        padding-top: 12px;
                        height: 20%;
                        
                        TextInput {
                            width: 80%;
                            height: root.height - 220px;
                            text: " At vero eos eor a clsum dolor sit amet.";
                            // 边框
                            // border-width: 1px;
                            // border-color: black;
                        }

                        Button {
                            padding-left: 10px;
                            width: 18%;
                            height: 50px;
                            text: "发送";
                            clicked => { root.show_confirm_popup(); }
                        }
                    }
                }
            }
        }

        

        confirm_popup := PopupWindow {
            x: 40px;
            y: 100px;
            height: 100px;
            width: root.width - 80px;
            Rectangle {
                background: root.background;
                border-color: confirm_popup_text.color;
                border-width: 2px;
            }
            confirm_popup_layout := Dialog {
                height:100%; width: 100%;
                confirm_popup_text := Text {
                    text: "发送消息?";
                    wrap: word-wrap;
                }
                StandardButton { kind: yes; clicked => { root.popup_confirmed(); } }
                StandardButton { kind: no; }
            }
        }


    }
}

fn main() {
    let main_window = HelloWorld::new().unwrap();

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    println!("run");

    // 隐藏之后，过几秒再次显示
    loop {
        main_window.run().unwrap();
        thread::sleep(std::time::Duration::from_secs(3));
    }
}
