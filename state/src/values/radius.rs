use crate::Parse;
use torin::radius::Radius;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRadiusError;

impl Parse for Radius {
    type Err = ParseRadiusError;

    fn parse(value: &str, scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        let mut radius = Radius::default();

        let mut values = value.split_ascii_whitespace();
        let scale_factor = scale_factor.unwrap_or(1.0);

        match values.clone().count() {
            // Same in all corners
            1 => {
                radius.fill_all(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor,
                );
            }
            // By Top and Bottom
            2 => {
                // Top
                radius.fill_top(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor
                );

                // Bottom
                radius.fill_bottom(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor
                )
            }
            // Each corner
            4 => {
                radius = Radius::new(
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor,
                    values
                        .next()
                        .ok_or(ParseRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseRadiusError)?
                        * scale_factor,
                );
            }
            _ => {}
        }

        Ok(radius)
    }
}