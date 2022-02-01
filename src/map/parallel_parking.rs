use crate::{CAR_HEIGHT, CAR_WIDTH, Rect, point2, Car, MAP_WIDTH, MAP_HEIGHT, Color, Source};

use super::Map;

const ROAD_WIDTH: f32 = CAR_WIDTH * 3.;
const PARKING_LENGTH: f32 = 6.7;
const PARKING_WIDTH: f32 = 3.0;

pub struct ParallelParking {
    road: Rect,
    parking_space: Rect,
}

impl ParallelParking {
    pub fn new() -> Self {
        let color = Color{r: 0xff, g: 0xff, b: 0xff};
        let road = Rect::new(point2(MAP_WIDTH/2., MAP_HEIGHT/2.), ROAD_WIDTH, MAP_HEIGHT, Source::Color(color));
        let parking_space = Rect::new(
            point2(road.origin.x+ROAD_WIDTH/2.+PARKING_WIDTH/2., road.origin.y),
            PARKING_WIDTH, PARKING_LENGTH, Source::Color(color));
        ParallelParking {
            road,
            parking_space,
        }
    }
}

impl Map for ParallelParking {
    fn car(&self) -> Car {
        Car::new(point2(self.road.origin.x, CAR_HEIGHT), 0.)
    }
}
