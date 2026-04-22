use cria::tui::app::state::App;

#[test]
fn test_show_and_hide_quick_add_modal() {
    let mut app = App::default();
    assert!(!app.show_quick_add_modal);
    app.show_quick_add_modal();
    assert!(app.show_quick_add_modal);
    assert_eq!(app.quick_add_input, "");
    assert_eq!(app.quick_add_cursor_position, 0);
    app.quick_add_input = "test".to_string();
    app.quick_add_cursor_position = 4;
    app.hide_quick_add_modal();
    assert!(!app.show_quick_add_modal);
    assert_eq!(app.quick_add_input, "");
    assert_eq!(app.quick_add_cursor_position, 0);
}

#[test]
fn test_add_and_delete_char_quick_add() {
    let mut app = App::default();
    app.show_quick_add_modal();
    app.add_char_to_quick_add('a');
    app.add_char_to_quick_add('b');
    app.add_char_to_quick_add('c');
    assert_eq!(app.quick_add_input, "abc");
    assert_eq!(app.quick_add_cursor_position, 3);
    app.delete_char_from_quick_add();
    assert_eq!(app.quick_add_input, "ab");
    assert_eq!(app.quick_add_cursor_position, 2);
}

#[test]
fn test_quick_add_cursor_movement() {
    let mut app = App::default();
    app.show_quick_add_modal();
    app.add_char_to_quick_add('x');
    app.add_char_to_quick_add('y');
    app.add_char_to_quick_add('z');
    assert_eq!(app.quick_add_cursor_position, 3);
    app.move_cursor_left();
    assert_eq!(app.quick_add_cursor_position, 2);
    app.move_cursor_left();
    assert_eq!(app.quick_add_cursor_position, 1);
    app.move_cursor_right();
    assert_eq!(app.quick_add_cursor_position, 2);
}

#[test]
fn test_quick_add_non_ascii_input_does_not_panic() {
    let mut app = App::default();
    app.show_quick_add_modal();

    for ch in "test non-ascii char à".chars() {
        app.add_char_to_quick_add(ch);
        let input = app.quick_add_input.clone();
        let cursor = app.quick_add_cursor_position;
        app.update_suggestions(&input, cursor);
    }

    assert_eq!(app.quick_add_input, "test non-ascii char à");
    assert_eq!(app.quick_add_cursor_position, "test non-ascii char à".chars().count());

    app.delete_char_from_quick_add();

    assert_eq!(app.quick_add_input, "test non-ascii char ");
    assert_eq!(app.quick_add_cursor_position, "test non-ascii char ".chars().count());
}
