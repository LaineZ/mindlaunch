pub mod drawing {
    use font_kit::font::Font;
    use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
    use std::time;

    const WIDTH: usize = 854;
    const HEIGHT: usize = 480;

    pub struct Drawing<'a> {
        pub dt: DrawTarget,
        pub font: &'a Font,
        window_w: f32,
        window_h: f32,
        // just for debug
        pub count: i32,
        // mouse
        mouse_x: f32,
        mouse_y: f32,
        pub last_click: time::Instant,
    }

    #[derive(Clone, Copy)]
    pub struct Location2 {
        pub x: f32,
        pub y: f32,
    }

    impl Location2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    impl From<Location4> for Location2 {
        fn from(location: Location4) -> Self {
            Self {
                x: location.x,
                y: location.y,
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Location4 {
        pub x: f32,
        pub y: f32,
        pub w: f32,
        pub h: f32,
    }

    impl Location4 {
        pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
            Self { x, y, w, h }
        }
    }
    #[derive(Clone, Copy)]
    pub struct Color {
        pub a: u8,
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
            Self { a, r, g, b }
        }
    }

    impl<'a> Drawing<'a> {
        pub fn new(mut dt: DrawTarget, font: &'a Font) -> Self {
            dt.clear(SolidSource::from_unpremultiplied_argb(
                255, 25, 25, 25,
            ));
            Self {
                dt,
                font,
                window_w: WIDTH as f32,
                window_h: HEIGHT as f32,
                count: 0,
                mouse_x: 0.0,
                mouse_y: 0.0,
                last_click: time::Instant::now(),
            }
        }

        pub fn process_mouse(&mut self, x: f32, y: f32) {
            self.mouse_x = x;
            self.mouse_y = y;
            //println!("m {}x{}", x, y);
        }

        pub fn check_click(
            &self,
            x1: f32,
            y1: f32,
            w1: f32,
            h1: f32,
            x2: f32,
            y2: f32,
            w2: f32,
            h2: f32,
        ) -> bool {
            return x1 < x2 + w2 && x2 < x1 + w1 && y1 < y2 + h2 && y2 < y1 + h1;
        }

        pub fn draw_square(&mut self, location: Location4, color: Color) {
            let mut pb = PathBuilder::new();
            pb.rect(location.x, location.y, location.w, location.h);
            let path = pb.finish();
            &self.dt.fill(
                &path,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    color.a, color.r, color.g, color.b,
                )),
                &DrawOptions::new(),
            );

            // for debug
            self.count += 1;
        }

        pub fn draw_button(&mut self, text: &str, location: Location4, color: Color) -> bool {
            if self.window_w > location.x + location.w
                && self.window_h > location.y + location.h / 8.0
                && 0.0 < location.y + location.h
            {
                self.draw_square(location, color);
                self.draw_text(
                    text,
                    Location2::new(location.x, location.y + 16.0),
                    Color::new(255, 0, 0, 0),
                    12.5,
                );

                //self.draw_text(&format!("location.x: {} location.y: {} location.w: {} location.h: {} || mx: {} my: {}", location.x, location.y, location.w, location.h, self.mouse_x, self.mouse_y), location.x, location.y);
            }

            return self.check_click(
                location.x,
                location.y,
                location.w,
                location.h,
                self.mouse_x,
                self.mouse_y,
                1.,
                1.,
            );
        }

        pub fn draw_text(&mut self, text: &str, location: Location2, color: Color, size: f32) {
            self.dt.draw_text(
                &self.font,
                size,
                text,
                Point::new(location.x, location.y),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(
                    color.a, color.r, color.g, color.b,
                )),
                &DrawOptions::new(),
            );

            // for debug
            self.count += 1;
        }


        pub fn draw_warn(&mut self, text: &str) {
            self.draw_square(Location4::new(0.0, self.window_h / 2.0, self.window_w, 100.), Color::new(127, 0, 0, 0))
        }
    }
}
