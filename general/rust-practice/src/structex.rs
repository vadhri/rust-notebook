#![allow(dead_code)]

pub struct Point {
    pub x: i32,
    pub y: i32
}

pub struct Line {
    pub start: Point,
    pub end: Point
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub background_color: Color
}

impl Circle {
    pub fn area(&self) -> f64 {
        self.radius.pow(2) as f64 * std::f64::consts::PI
    }
}

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    scores_sorted: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores_vector = scores.to_vec();
        let mut sorted_scores = scores_vector.clone();
        sorted_scores.sort();
        sorted_scores.reverse();

        HighScores {
            scores: scores_vector,
            scores_sorted: sorted_scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_sorted.first().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let max_scores = self.scores_sorted.len();
        match max_scores {
            x if x >= 3 => self.scores_sorted[0..=2].to_vec(),
            _ => self.scores_sorted[0..max_scores].to_vec()
        }
    }
}


#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    RgbColorTriplet(u8, u8, u8)
}

pub struct Rectangle {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point
}

pub fn get_circle(c: Color, p: Point, r: i32) -> Circle {
    Circle { center: p, radius: r, background_color: c }
}
