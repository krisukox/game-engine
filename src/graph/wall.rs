use super::Coordinate;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Wall {
    pub start_point: Coordinate,
    pub end_point: Coordinate,
}

impl Wall {
    pub fn point_distance_start(&self, point: &Coordinate) -> f64 {
        point.distance(&self.start_point)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Walls(pub Vec<Wall>);

pub enum TryExtendValue {
    Extended,
    ContainsAlready,
    OnePoint(Coordinate),
    NewWall,
    NoPoint,
    NoWalls,
}

impl Walls {
    pub fn try_extend_last_wall(&mut self, points: &mut Vec<Coordinate>) -> TryExtendValue {
        if let Some(last_wall) = self.0.last_mut() {
            if let Some(first) = points.get(0) {
                if *first == last_wall.start_point {
                    return TryExtendValue::ContainsAlready;
                }
                if let Some(second) = points.get(1) {
                    if (last_wall.end_point == *first && last_wall.start_point != *second)
                        || (last_wall.end_point == *second && last_wall.start_point != *first)
                    {
                        if (last_wall.start_point.x == last_wall.end_point.x
                            && last_wall.end_point.x == second.x)
                            || (last_wall.start_point.y == last_wall.end_point.y
                                && last_wall.end_point.y == second.y)
                        {
                            last_wall.end_point = second.clone();
                            return TryExtendValue::Extended;
                        }
                    }
                    self.0.push(Wall {
                        start_point: first.clone(),
                        end_point: second.clone(),
                    });
                    return TryExtendValue::NewWall;
                }
                return TryExtendValue::OnePoint(points.remove(0));
            }
        } else {
            if let Some(first) = points.get(0) {
                if let Some(second) = points.get(1) {
                    self.0.push(Wall {
                        start_point: first.clone(),
                        end_point: second.clone(),
                    });
                    return TryExtendValue::NewWall;
                }
                return TryExtendValue::OnePoint(points.remove(0));
            }
        }
        return TryExtendValue::NoPoint;
    }
}
