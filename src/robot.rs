use std::f32::consts::PI;
use svg::node;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub struct Robot {
    document: svg::Document,
    color: String,
    data: Option<Data>,
    x: f32,
    y: f32,
    angle: f32,
    labelheight: f32,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            document: Document::new(),
            color: "black".to_string(),
            data: Some(Data::new().move_to((500, 500))),
            x: 250.0,
            y: 250.0,
            angle: -PI / 2.0,
            labelheight: 100.,
        }
    }

    pub fn home(&mut self) {
        self.setpos(500.0, 500.0);
    }

    pub fn forward(&mut self, m: f32) {
        let x: f32 = self.angle.cos() * m;
        let y: f32 = self.angle.sin() * m;
        self.data = self.data.take().map(|d| d.line_by((x, y)));
        self.x += x;
        self.y += y;
    }

    fn flush_path(&mut self) {
        if let Some(data) = self.data.clone() {
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", self.color.clone())
                .set("stroke-width", 1)
                .set("d", data);
            self.document = self.document.clone().add(path);
            self.data = Some(Data::new().move_to((self.x, self.y)));
        }
    }

    pub fn setpencolor(&mut self, color: String) {
        self.flush_path();
        self.color = color;
    }

    pub fn penup(&mut self) {
        self.flush_path();
        self.data = None;
    }

    pub fn pendown(&mut self) {
        self.flush_path();
        self.data = Some(Data::new().move_to((self.x, self.y)));
    }

    pub fn back(&mut self, m: f32) {
        self.forward(-m);
    }

    pub fn left(&mut self, t: f32) {
        self.angle -= t;
    }

    pub fn right(&mut self, t: f32) {
        self.angle += t;
    }

    pub fn plot(&mut self) -> svg::Document {
        self.flush_path();

        self.document.clone().set("viewBox", (0, 0, 1000, 1000))
    }

    pub fn setpos(&mut self, x: f32, y: f32) {
        if let Some(data) = self.data.clone() {
            self.data = Some(data.move_to((x, y)));
        }
        self.x = x;
        self.y = y;
    }

    pub fn label(&mut self, s: String) {
        let txt = node::element::Text::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("fill", &*self.color)
            .set(
                "transform",
                format!("rotate({} {},{})", 180.0 * self.angle / PI, self.x, self.y),
            )
            .set("style", format!("font: {}px sans-serif;", self.labelheight))
            .add(node::Text::new(s));

        self.document = self.document.clone().add(txt);
    }

    pub fn setlabelheight(&mut self, h: f32) {
        self.labelheight = h;
    }

    pub fn clean(&mut self) {
        self.flush_path();
        self.document = Document::new();
    }

    pub fn clearscreen(&mut self) {
        self.home();
        self.clean();
    }
}
