use pulse::{Frame, Rect};

fn main() {
    let mut frame = Frame::new(8, 3);

    assert_eq!(frame.width(), 8);
    assert_eq!(frame.height(), 3);

    frame.print(0, 0, "abc");
    assert_eq!(frame.char_at(0, 0), Some('a'));
    assert_eq!(frame.char_at(1, 0), Some('b'));
    assert_eq!(frame.char_at(2, 0), Some('c'));

    frame.print(7, 1, "XYZ");
    assert_eq!(frame.char_at(7, 1), Some('X'));
    assert_eq!(frame.char_at(6, 1), Some(' '));

    frame.print(20, 20, "out");
    assert_eq!(frame.char_at(0, 2), Some(' '));

    frame.clear();
    frame.render_in(Rect::new(2, 1, 3, 1), |f| {
        f.print(0, 0, "hello");
        f.print(4, 0, "x");
        f.print(0, 1, "y");
    });
    assert_eq!(frame.char_at(2, 1), Some('h'));
    assert_eq!(frame.char_at(3, 1), Some('e'));
    assert_eq!(frame.char_at(4, 1), Some('l'));
    assert_eq!(frame.char_at(5, 1), Some(' '));
    assert_eq!(frame.char_at(2, 2), Some(' '));

    frame.clear();
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            assert_eq!(frame.char_at(x, y), Some(' '));
        }
    }

    println!("frame_check: ok");
}
