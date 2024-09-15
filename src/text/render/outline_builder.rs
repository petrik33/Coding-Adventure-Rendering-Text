pub struct OutlineBuilder {
    path_builder: Option<tiny_skia::PathBuilder>,
    pub contours: Vec<tiny_skia::Path>,
}

impl OutlineBuilder {
    pub fn new() -> Self {
        OutlineBuilder {
            path_builder: None,
            contours: Vec::new(),
        }
    }

    const EXPECT_CONTOUR_STARTED_MSG: &'static str =
        "Contour outline should be first moved to start point with `move_to` method call";
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_builder = Some(tiny_skia::PathBuilder::new());
        self.path_builder.as_mut().unwrap().move_to(x, y)
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path_builder
            .as_mut()
            .expect(Self::EXPECT_CONTOUR_STARTED_MSG)
            .line_to(x, y)
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path_builder
            .as_mut()
            .expect(Self::EXPECT_CONTOUR_STARTED_MSG)
            .quad_to(x1, y1, x, y)
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path_builder
            .as_mut()
            .expect(Self::EXPECT_CONTOUR_STARTED_MSG)
            .cubic_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        if let Some(contour) = self
            .path_builder
            .take()
            .expect(Self::EXPECT_CONTOUR_STARTED_MSG)
            .finish()
        {
            self.contours.push(contour)
        }
    }
}
