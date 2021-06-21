use super::Coordinate;
use crate::map_element::{Color, Point};

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Wall {
    pub start_point: Point,
    pub end_point: Point,
    pub primary_object_color: Color,
}

impl Wall {
    pub fn point_distance_start(&self, coordinate: &Coordinate) -> f64 {
        self.start_point.distance_coor(&coordinate)
    }

    pub fn point_distance_end(&self, coordinate: &Coordinate) -> f64 {
        self.end_point.distance_coor(&coordinate)
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Walls(pub Vec<Wall>);

impl Walls {
    pub fn try_extend_last_wall(&mut self, wall: Wall) {
        if let Some(last_wall) = self.0.last_mut() {
            if last_wall.start_point == wall.start_point {
                return;
            }
            if last_wall.end_point == wall.start_point
                && last_wall.primary_object_color == wall.primary_object_color
            {
                if (last_wall.start_point.x == last_wall.end_point.x
                    && last_wall.end_point.x == wall.end_point.x)
                    || (last_wall.start_point.y == last_wall.end_point.y
                        && last_wall.end_point.y == wall.end_point.y)
                {
                    last_wall.end_point = wall.end_point;
                    return;
                }
            }
            self.0.push(wall);
        } else {
            self.0.push(wall);
        }
    }

    pub fn merge(&mut self, mut walls_to_merge: Walls) {
        if let Some(wall) = self.0.last_mut() {
            if walls_to_merge.0.len() > 0 {
                let wall_to_merge = walls_to_merge.0.remove(0);
                if wall.primary_object_color != wall_to_merge.primary_object_color {
                    self.0.push(wall_to_merge);
                } else if wall.start_point.x == wall.end_point.x
                    && wall.end_point.x == wall_to_merge.start_point.x
                    && wall_to_merge.start_point.x == wall_to_merge.end_point.x
                {
                    if wall.start_point.y <= wall_to_merge.start_point.y
                        && wall_to_merge.start_point.y <= wall.end_point.y
                    {
                        wall.end_point = wall_to_merge.end_point;
                    } else if wall.end_point.y <= wall_to_merge.start_point.y
                        && wall_to_merge.start_point.y <= wall.start_point.y
                    {
                        wall.end_point = wall_to_merge.end_point;
                    }
                } else if wall.start_point.y == wall.end_point.y
                    && wall.end_point.y == wall_to_merge.start_point.y
                    && wall_to_merge.start_point.y == wall_to_merge.end_point.y
                {
                    if wall.start_point.x <= wall_to_merge.start_point.x
                        && wall_to_merge.start_point.x <= wall.end_point.x
                    {
                        wall.end_point = wall_to_merge.end_point;
                    } else if wall.end_point.x <= wall_to_merge.start_point.x
                        && wall_to_merge.start_point.x <= wall.start_point.x
                    {
                        wall.end_point = wall_to_merge.end_point;
                    }
                } else {
                    self.0.push(wall_to_merge);
                }
            }
        }
        self.0.append(&mut walls_to_merge.0);
    }
}
