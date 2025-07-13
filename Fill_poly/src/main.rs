use raylib::prelude::*;

fn draw_line(p0: (i32, i32), p1: (i32, i32), image: &mut Image, color: Color) {
    let (mut x0, mut y0) = p0;
    let (x1, y1) = p1;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut e2;

    loop {
        image.draw_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_poli(polygon: &Vec<(i32, i32)>, image: &mut Image, color: Color) {
    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        draw_line(polygon[i], polygon[j], image, color);
    }
}

fn fill_poli(polygon: &Vec<(i32, i32)>, image: &mut Image, border_color: Color, fill_color: Color) {
    let (xmin, ymin, xmax, ymax) = bounding_box(polygon);
    let data = image.get_image_data();
    let width = image.width();

    for y in ymin..=ymax {
        let mut activo = false;
        let mut last_pixel_was_border = false;

        for x in xmin..=xmax {
            let idx = (y * width + x) as usize;
            if idx >= data.len() {
                continue;
            }

            let pixel = data[idx];

            if pixel == border_color {
                if !last_pixel_was_border {
                    activo = !activo;
                    last_pixel_was_border = true;
                }
            } else {
                last_pixel_was_border = false;
                if activo {
                    image.draw_pixel(x, y, fill_color);
                }
            }
        }
    }
}

fn bounding_box(polygon: &Vec<(i32, i32)>) -> (i32, i32, i32, i32) {
    let (mut xmin, mut ymin) = polygon[0];
    let (mut xmax, mut ymax) = (xmin, ymin);
    for &(x, y) in polygon.iter() {
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        ymin = ymin.min(y);
        ymax = ymax.max(y);
    }
    (xmin, ymin, xmax, ymax)
}

fn main() {
    let poligono1 = vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];
    let poligono2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    let poligono3 = vec![(377, 249), (411, 197), (436, 249)];
    let poligono4 = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ];
    let poligono5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)];

    let width = 1080;
    let height = 720;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Polígonos rellenados")
        .build();

    let mut image = Image::gen_image_color(width, height, Color::BLACK);

    // Dibujar y rellenar polígonos
    draw_poli(&poligono1, &mut image, Color::BLUE);
    fill_poli(&poligono1, &mut image, Color::BLUE, Color::SKYBLUE);

    draw_poli(&poligono2, &mut image, Color::RED);
    fill_poli(&poligono2, &mut image, Color::RED, Color::PINK);

    draw_poli(&poligono3, &mut image, Color::GREEN);
    fill_poli(&poligono3, &mut image, Color::GREEN, Color::LIME);

    draw_poli(&poligono4, &mut image, Color::YELLOW);

    draw_poli(&poligono5, &mut image, Color::YELLOW);
    //fill_poli(&poligono5, &mut image, Color::PURPLE, Color::VIOLET);
    fill_poli(&poligono4, &mut image, Color::YELLOW, Color::ORANGE);

    let texture = rl.load_texture_from_image(&thread, &image).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
    let output_file_name = "Figuras_rellenas.png";
    image.export_image(output_file_name);

    println!("Imagen guardada con éxito como '{}'!", output_file_name);
}
