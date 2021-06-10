use super::Coordinate;
use crate::map_element::{Color, ColoredPoint, Point};

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
    pub fn try_extend_last_wall(&mut self, points: &mut Vec<ColoredPoint>) -> Option<ColoredPoint> {
        if let Some(last_wall) = self.0.last_mut() {
            if let Some(first) = points.get(0) {
                if let Some(second) = points.get(1) {
                    if second.point == last_wall.end_point {
                        return None;
                    }
                    if last_wall.end_point == first.point && last_wall.start_point != second.point {
                        if (last_wall.start_point.x == last_wall.end_point.x
                            && last_wall.end_point.x == second.point.x)
                            || (last_wall.start_point.y == last_wall.end_point.y
                                && last_wall.end_point.y == second.point.y)
                        {
                            if last_wall.primary_object_color == first.color {
                                last_wall.end_point = second.point.clone();
                                return None;
                            } else if first.color == second.color {
                                last_wall.end_point = first.point.clone();
                                self.0.push(Wall {
                                    start_point: first.point.clone(),
                                    end_point: second.point.clone(),
                                    primary_object_color: first.color.clone(),
                                });
                                return None;
                            }
                        }
                    }
                    self.0.push(Wall {
                        start_point: first.point.clone(),
                        end_point: second.point.clone(),
                        primary_object_color: second.color.clone(),
                    });
                    return None;
                }
                return Some(points.remove(0));
            }
        } else {
            if let Some(first) = points.get(0) {
                if let Some(second) = points.get(1) {
                    self.0.push(Wall {
                        start_point: first.point.clone(),
                        end_point: second.point.clone(),
                        primary_object_color: first.color.clone(),
                    });
                    return None;
                }
                return Some(points.remove(0));
            }
        }
        return None;
    }

    pub fn merge(&mut self, mut walls_to_merge: Walls) {
        if let Some(wall) = self.0.last_mut() {
            if walls_to_merge.0.len() > 0 {
                let wall_to_merge = walls_to_merge.0.remove(0);
                if wall.start_point.x == wall.end_point.x
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
