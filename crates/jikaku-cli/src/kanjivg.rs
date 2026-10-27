use anyhow::{Context, Result, anyhow};
use roxmltree::{Document, Node, ParsingOptions};

#[derive(Debug)]
pub struct Character<T> {
    pub id: u32,
    pub strokes: Vec<Stroke<T>>,
}

impl Character<f32> {
    pub fn normalize(self, grid_size: u16) -> Character<u8> {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = 0.0;
        let mut max_y = 0.0;

        for point in self.strokes.iter().flat_map(|s| &s.points) {
            min_x = point.x.min(min_x);
            min_y = point.y.min(min_y);
            max_x = point.x.max(max_x);
            max_y = point.y.max(max_y);
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        let vratio = grid_size as f32 / width;
        let hratio = grid_size as f32 / height;
        let ratio = vratio.min(hratio);

        let strokes = self
            .strokes
            .into_iter()
            .map(|s| s.map(min_x, min_y, ratio))
            .collect();

        Character {
            id: self.id,
            strokes,
        }
    }
}

impl Character<u8> {
    pub fn simplify(self, min_distance: u16) -> Self {
        Self {
            id: self.id,
            strokes: self
                .strokes
                .into_iter()
                .map(|s| s.simplify(min_distance))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Stroke<T> {
    pub points: Vec<Point<T>>,
}

impl Stroke<f32> {
    pub fn map(self, x_start: f32, y_start: f32, ratio: f32) -> Stroke<u8> {
        let points = self
            .points
            .iter()
            .map(|p| {
                let x = ((p.x - x_start) * ratio).round().min(255.0) as u8;
                let y = ((p.y - y_start) * ratio).round().min(255.0) as u8;

                Point { x, y }
            })
            .collect();

        Stroke { points }
    }
}

impl Stroke<u8> {
    pub fn simplify(self, min_distance: u16) -> Self {
        let mut points = Vec::new();
        let min_distance = min_distance.pow(2);

        for (i, point) in self.points.iter().enumerate() {
            if i == 0 || i == self.points.len() - 1 {
                points.push(*point);
            } else if let Some(last) = points.last() {
                let distance = (last.x as i32 - point.x as i32).pow(2)
                    + (last.y as i32 - point.y as i32).pow(2);

                if distance > min_distance as i32 {
                    points.push(*point);
                }
            }
        }

        Stroke { points }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Absolute,
    Relative,
}

pub fn parse_kvg(content: &str, curve_steps: u16) -> Result<Character<f32>> {
    let parsing_options = ParsingOptions {
        allow_dtd: true,
        ..Default::default()
    };

    let tree = Document::parse_with_options(content, parsing_options)?;
    let svg_elem = tree.root_element();

    let kvg_elem = svg_elem
        .descendants()
        .find(|c| c.has_attribute("element"))
        .context("expected tag with 'kvg:element' attribute")?;

    let id = kvg_elem
        .attribute("element")
        .and_then(|ch| ch.chars().next().map(|ch| ch as u32))
        .context("invalid 'kvg:element' attribute")?;

    let strokes = kvg_elem
        .descendants()
        .filter(|d| d.has_tag_name("path"))
        .map(|d| resolve_path(d, curve_steps))
        .collect::<Result<Vec<_>>>()?;

    Ok(Character { id, strokes })
}

fn resolve_path(node: Node, curve_steps: u16) -> Result<Stroke<f32>> {
    let commands = node.attribute("d").context("missing 'd' attribute")?;
    let resolver = PathResolver::new(commands, curve_steps);
    let points = resolver.resolve()?;

    Ok(Stroke { points })
}

#[derive(Debug)]
struct PathResolver<'a> {
    commands: &'a str,
    curve_steps: u16,
    position: Point<f32>,
    mirror: Point<f32>,
    points: Vec<Point<f32>>,
}

impl<'a> PathResolver<'a> {
    fn new(commands: &'a str, curve_steps: u16) -> Self {
        Self {
            position: Point::default(),
            mirror: Point::default(),
            points: Vec::new(),
            curve_steps,
            commands,
        }
    }

    fn resolve(mut self) -> Result<Vec<Point<f32>>> {
        while let Some(cmd) = self.commands.chars().next() {
            self.commands = &self.commands[1..];

            match cmd {
                'm' => self.position = self.take_point(Position::Relative)?,
                'M' => self.position = self.take_point(Position::Absolute)?,
                'c' => self.handle_curve(Position::Relative, false)?,
                'C' => self.handle_curve(Position::Absolute, false)?,
                's' => self.handle_curve(Position::Relative, true)?,
                'S' => self.handle_curve(Position::Absolute, true)?,
                _ => return Err(anyhow!("unhandled path command: '{cmd}'")),
            }
        }

        Ok(self.points)
    }

    fn handle_curve(&mut self, position: Position, smooth: bool) -> Result<()> {
        let p0 = self.position;

        let p1 = if smooth {
            Point {
                x: (self.position.x - self.mirror.x) + self.position.x,
                y: (self.position.y - self.mirror.y) + self.position.y,
            }
        } else {
            self.take_point(position)?
        };

        let p2 = self.take_point(position)?;
        let p3 = self.take_point(position)?;

        for i in 0..=self.curve_steps {
            let t = i as f32 / self.curve_steps as f32;
            let x = self.interpolate(t, p0.x, p1.x, p2.x, p3.x);
            let y = self.interpolate(t, p0.y, p1.y, p2.y, p3.y);

            self.points.push(Point { x, y });
        }

        self.mirror = p2;
        self.position = p3;

        if self
            .commands
            .chars()
            .next()
            .is_some_and(|ch| ch.is_numeric() || ch == '-')
        {
            self.handle_curve(position, smooth)?;
        }

        Ok(())
    }

    fn take_point(&mut self, position: Position) -> Result<Point<f32>> {
        let mut x = self.take_number()?;
        let mut y = self.take_number()?;

        if matches!(position, Position::Relative) {
            x += self.position.x;
            y += self.position.y;
        }

        Ok(Point { x, y })
    }

    fn skip_separator(&mut self) {
        let length = self
            .commands
            .chars()
            .take_while(|&ch| ch == ',' || ch == ' ')
            .count();

        self.commands = &self.commands[length..]
    }

    fn take_number(&mut self) -> Result<f32> {
        self.skip_separator();

        let mut offset = 0;

        if self.commands.starts_with("-") {
            offset += 1;
        }

        let digits = self.commands[offset..]
            .chars()
            .take_while(|&ch| ch.is_numeric() || ch == '.');

        offset += digits.count();

        let num_str = &self.commands[..offset];
        let parsed = num_str.parse()?;

        self.commands = &self.commands[offset..];
        self.skip_separator();

        Ok(parsed)
    }

    fn interpolate(&self, t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
        let r1 = -p0 * t.powi(3) + 3.0 * p0 * t.powi(2) - 3.0 * p0 * t + p0;
        let r2 = 3.0 * p1 * t.powi(3) - 6.0 * p1 * t.powi(2) + 3.0 * p1 * t;
        let r3 = -3.0 * p2 * t.powi(3) + 3.0 * p2 * t.powi(2);
        let r4 = p3 * t.powi(3);

        r1 + r2 + r3 + r4
    }
}
