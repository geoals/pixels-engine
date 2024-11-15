use crate::{HEIGHT, TILE_SIZE, WIDTH};

/// Draw horizontal or vertical line pixel by pixel into frame
pub fn line(from: (u32, u32), to: (u32, u32), frame: &mut [u8]) {
    let (x0, y0) = from;
    let (x1, y1) = to;

    if x0 == x1 {
        // Vertical line
        let y_start = y0.min(y1);
        let y_end = y0.max(y1);

        for y in y_start..y_end {
            let index = (x0 + y * WIDTH) as usize * 4;
            // Set RGBA to white fully opaque
            frame[index] = 255;
            frame[index + 1] = 255;
            frame[index + 2] = 255;
            frame[index + 3] = 255;
        }
    } else if y0 == y1 {
        // Horizontal line
        let x_start = x0.min(x1);
        let x_end = x0.max(x1);

        for x in x_start..x_end {
            let index = (x + y0 * WIDTH) as usize * 4;
            // Set RGBA to white fully opaque
            frame[index] = 255;
            frame[index + 1] = 255;
            frame[index + 2] = 255;
            frame[index + 3] = 255;
        }
    }
}

pub fn draw_grid(frame: &mut [u8]) {
    for x in 1..(WIDTH / TILE_SIZE) {
        line((x * TILE_SIZE, 0), (x * TILE_SIZE, HEIGHT), frame);
    }
    for y in 0..(HEIGHT / TILE_SIZE) {
        line((0, y * TILE_SIZE), (WIDTH, y * TILE_SIZE), frame);
    }
}
