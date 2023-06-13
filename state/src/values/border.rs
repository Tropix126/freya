#[derive(Default, Clone, Debug, PartialEq)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub color: Color,
    pub style: BorderStyle,
    pub width: f32,
    pub alignment: BorderAlignment,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Border {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}