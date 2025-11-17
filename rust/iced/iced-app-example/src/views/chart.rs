use iced::widget::{column, container, text, Column};

use crate::app::App;
use crate::message::Message;

pub fn create_chart_view(app: &App) -> Column<'_, Message> {
    column![
        text("Chart View").size(32),
        container(
            column![
                text("Data Visualization").size(24),
                app.chart.view(),
                text(format!("Counter Value: {}", app.value)).size(16),
                text(format!("Age: {}", app.age)).size(16),
                text(format!("Experience: {:.1} years", app.experience)).size(16),
            ]
            .spacing(20)
        )
        .padding(20),
    ]
    .spacing(20)
    .padding(20)
}
