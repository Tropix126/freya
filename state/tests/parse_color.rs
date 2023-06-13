use freya_node_state::parse_color;
use skia_safe::Color;

#[test]
fn parse_manual_color() {
    let color = parse_color("red");
    assert_eq!(color, Some(Color::RED));
}

#[test]
fn parse_rgb_color() {
    let color = parse_color("rgb(91, 123, 57)");
    assert_eq!(color, Some(Color::from_rgb(91, 123, 57)));
}

#[test]
fn parse_rgb_color_with_alpha() {
    let color = parse_color("rgb(91, 123, 57, 127)");
    assert_eq!(color, Some(Color::from_argb(127, 91, 123, 57)));
}
