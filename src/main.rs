use slint::{LogicalPosition, private_unstable_api::re_exports::PointerEventKind, Weak};

slint::include_modules!();

fn main() {
    let main_window = Main::new().unwrap();
    

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    handle_window_resize(main_window.as_weak());
    
    main_window.run().unwrap();
}

/// 处理鼠标移动和缩放窗口
fn handle_window_resize(handle: Weak<Main>){
    let main_window = match handle.upgrade(){
        Some(main) => main,
        None => return
    };

    let handle1 = handle.clone();
    main_window.on_exit(move ||{
        let app = match handle1.upgrade(){
            None => return,
            Some(v) => v
        };
        let _ = app.window().hide();
    });

    main_window.on_drag_area_mouse_event(move |_pos, ev, mouse_x, mouse_y|{
        let app = match handle.upgrade(){
            None => return,
            Some(v) => v
        };
        app.set_drag_area_pressed(ev.kind == PointerEventKind::Down);
        app.set_drag_area_down_x(mouse_x);
        app.set_drag_area_down_y(mouse_y);
    });
    
    let handle = main_window.as_weak();
    main_window.on_drag_area_mouse_move(move |pos, mouse_x, mouse_y|{
        let app = match handle.upgrade(){
            None => return,
            Some(v) => v
        };
        let pressed_x = app.get_drag_area_down_x();
        let pressed_y = app.get_drag_area_down_y();
        if app.get_drag_area_pressed() && pressed_x != mouse_x && pressed_y != mouse_y{
            let main_window = app.window();
            if pos == 0{
                //拖动窗口
                let logical_pos = main_window.position().to_logical(main_window.scale_factor());
                main_window.set_position(LogicalPosition::new(logical_pos.x + (mouse_x - pressed_x), logical_pos.y + (mouse_y - pressed_y)));
            }else{
                let mut logical_size = main_window.size().to_logical(main_window.scale_factor());
                if pos == 1{
                    //右边缩放窗口
                    logical_size.width = logical_size.width + (mouse_x - pressed_x);
                }else if pos == 2{
                    //右边缩放窗口
                    logical_size.width = logical_size.width + (mouse_x - pressed_x);
                    logical_size.height = logical_size.height + (mouse_y - pressed_y);
                }else if pos == 3{
                    //下边缩放窗口
                    logical_size.height = logical_size.height + (mouse_y - pressed_y);
                }
                logical_size.width = logical_size.width.max(300.);
                logical_size.height = logical_size.height.max(300.);
                main_window.set_size(logical_size);
            }
        }
    });
}