use crate::egui;

// length, width
const ROBOT_ONE: [f64; 2] = [0.381, 0.3302];
const HALF_ROBOT_ONE: [f64; 2] = [0.5 * ROBOT_ONE[0], 0.5 * ROBOT_ONE[1]];
const ROBOT_TWO: [f64; 2] = ROBOT_ONE;
const HALF_ROBOT_TWO: [f64; 2] = [0.5 * ROBOT_TWO[0], 0.5 * ROBOT_TWO[1]];

const POS_OFFSET: [f64; 2] = [0.0, 1.5];

pub struct Robot {
    first: bool,
    pos: [f64; 2],
    heading: f64,
}

impl Robot {
    pub fn new(first: bool, pos: [f64; 2], heading: f64) -> Self {
        Self {
            first,
            pos,
            heading,
        }
    }
    pub fn draw(&self, plot_ui: &mut egui_plot::PlotUi) {
        /*let (s, c) = (0.0, 1.0); //self.heading.sin_cos();

        let hd = if self.first {
            HALF_ROBOT_ONE
        } else {
            HALF_ROBOT_TWO
        };

        // points of the robot with respect to the robot centre
        // [fl, fr, br, bl]
        let mut points = [
            [hd[0], hd[1]],
            [hd[0], -hd[1]],
            [-hd[0], -hd[1]],
            [-hd[0], hd[1]],
        ];

        let mut indiciator_points = [
            [hd[0], 0.2 * hd[1]],
            [hd[0], -0.2 * hd[1]],
            [1.1 * hd[0], 0.0],
        ];

        // rotate around centre then offset by centre
        let rotate = |v: [f64; 2]| [c * v[0] - s * v[1], c * v[1] + s * v[0]];
        points = points.map(|p| {
            let p = rotate(p);
            [
                p[0] + self.pos[0] + POS_OFFSET[0],
                p[1] + self.pos[1] + POS_OFFSET[1],
            ]
        });
        indiciator_points = indiciator_points.map(|p| {
            let p = rotate(p);
            [
                p[0] + self.pos[0] + POS_OFFSET[0],
                p[1] + self.pos[1] + POS_OFFSET[1],
            ]
        });

        plot_ui.polygon(egui_plot::Polygon::new(points.to_vec()));
        plot_ui.polygon(
            egui_plot::Polygon::new(indiciator_points.to_vec())
                .stroke(egui::Stroke::new(1.0, egui::Color32::GREEN)),
        );*/
        plot_ui.points(
            egui_plot::Points::new([self.pos[0] + POS_OFFSET[0], self.pos[1] + POS_OFFSET[1]])
                .color(egui::Color32::GREEN)
                .radius(3.0),
        );
    }
}
