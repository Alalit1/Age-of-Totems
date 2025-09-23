use pixels::wgpu::Color;


pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color_button: (u8, u8, u8, u8),
    pub color_border: (u8, u8, u8, u8),
    pub color_button_hover: (u8, u8, u8, u8),
    pub color_border_hover: (u8, u8, u8, u8),
    pub border_radius: u8,

//    text: String
//    font: String
}

impl Button {
    pub fn draw(&self, frame: &mut [u8], screen_width: u32) {
        for row in self.y..(self.y + self.height as i32) {
            for col in self.x..(self.x + self.width as i32) {
                if row >= 0 && col >= 0 {
                    let idx = ((row as u32 * screen_width) + col as u32) as usize * 4;
                    if idx + 3 < frame.len() {
                        frame[idx] = self.color_button.0;     // R
                        frame[idx + 1] = self.color_button.1; // G
                        frame[idx + 2] = self.color_button.2; // B
                        frame[idx + 3] = self.color_button.3; // A
                    }
                }
            }
        }
    }
}
