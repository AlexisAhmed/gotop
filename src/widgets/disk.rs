use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct DiskWidget {
    title: String,
    update_interval: Duration,
}

impl DiskWidget {
    pub fn new() -> DiskWidget {
        DiskWidget {
            title: " Disk Usage ".to_string(),
            update_interval: Duration::from_secs(1),
        }
    }
}

impl Widget for DiskWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
