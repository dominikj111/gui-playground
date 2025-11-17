use iced::widget::canvas::{self, Cache, Canvas, Geometry, Path, Stroke, Text};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};

pub struct BarChart {
    cache: Cache,
    data: Vec<BarData>,
}

#[derive(Debug, Clone)]
pub struct BarData {
    pub label: String,
    pub value: f32,
    pub color: Color,
}

impl Clone for BarChart {
    fn clone(&self) -> Self {
        Self {
            cache: Cache::default(),
            data: self.data.clone(),
        }
    }
}

impl BarChart {
    pub fn new(data: Vec<BarData>) -> Self {
        Self {
            cache: Cache::default(),
            data,
        }
    }

    pub fn update_data(&mut self, data: Vec<BarData>) {
        self.data = data;
        self.cache.clear();
    }

    pub fn view<Message>(&self) -> Canvas<BarChart, Message, Theme, Renderer> 
    where 
        Message: 'static,
    {
        Canvas::new(self.clone())
            .width(600)
            .height(300)
    }
}

impl<Message> canvas::Program<Message, Theme, Renderer> for BarChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let padding = 40.0;
            let chart_width = bounds.width - padding * 2.0;
            let chart_height = bounds.height - padding * 2.0;

            // Draw background
            let background = Path::rectangle(Point::ORIGIN, bounds.size());
            frame.fill(&background, Color::from_rgb(0.95, 0.95, 0.95));

            if self.data.is_empty() {
                return;
            }

            // Find max value for scaling
            let max_value = self
                .data
                .iter()
                .map(|d| d.value)
                .fold(0.0f32, f32::max)
                .max(1.0);

            let bar_width = chart_width / self.data.len() as f32 * 0.7;
            let bar_spacing = chart_width / self.data.len() as f32;

            // Draw bars
            for (i, bar_data) in self.data.iter().enumerate() {
                let bar_height = (bar_data.value / max_value) * chart_height;
                let x = padding + i as f32 * bar_spacing + (bar_spacing - bar_width) / 2.0;
                let y = padding + chart_height - bar_height;

                // Draw bar
                let bar = Path::rectangle(Point::new(x, y), Size::new(bar_width, bar_height));
                frame.fill(&bar, bar_data.color);

                // Draw bar outline
                frame.stroke(
                    &bar,
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgb(0.3, 0.3, 0.3)),
                );

                // Draw value on top of bar
                let value_text = Text {
                    content: format!("{:.0}", bar_data.value),
                    position: Point::new(x + bar_width / 2.0, y - 15.0),
                    color: Color::BLACK,
                    size: 14.0.into(),
                    ..Text::default()
                };
                frame.fill_text(value_text);

                // Draw label below bar
                let label_text = Text {
                    content: bar_data.label.clone(),
                    position: Point::new(
                        x + bar_width / 2.0,
                        padding + chart_height + 10.0,
                    ),
                    color: Color::BLACK,
                    size: 12.0.into(),
                    ..Text::default()
                };
                frame.fill_text(label_text);
            }

            // Draw axes
            let x_axis = Path::line(
                Point::new(padding, padding + chart_height),
                Point::new(padding + chart_width, padding + chart_height),
            );
            frame.stroke(
                &x_axis,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::BLACK),
            );

            let y_axis = Path::line(
                Point::new(padding, padding),
                Point::new(padding, padding + chart_height),
            );
            frame.stroke(
                &y_axis,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::BLACK),
            );
        });

        vec![geometry]
    }
}
