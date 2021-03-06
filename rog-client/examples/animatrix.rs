use rog_client::anime_dbus::AniMeDbusWriter;
use rog_client::anime_matrix::{AniMeMatrix, AniMePacketType, HEIGHT, WIDTH};
use tinybmp::{Bmp, Pixel};

fn main() {
    let mut writer = AniMeDbusWriter::new().unwrap();

    let bmp =
        Bmp::from_slice(include_bytes!("non-skewed_r.bmp")).expect("Failed to parse BMP image");
    let pixels: Vec<Pixel> = bmp.into_iter().collect();
    //assert_eq!(pixels.len(), 56 * 56);

    // Try an outline, top and right
    let mut matrix = AniMeMatrix::new();

    // Aligned left
    for px in pixels {
        if (px.x as usize / 2) < WIDTH && (px.y as usize) < HEIGHT {
            if px.x % 2 == 0 {
                matrix.get_mut()[px.y as usize][px.x as usize / 2] = px.color as u8;
            }
        }
    }

    // Throw an alignment border up
    // {
    //     let tmp = matrix.get_mut();
    //     for x in tmp[0].iter_mut() {
    //         *x = 0xff;
    //     }
    //     for row in tmp.iter_mut() {
    //         row[row.len() - 1] = 0xff;
    //     }
    // }

    matrix.debug_print();

    let mut matrix: AniMePacketType = AniMePacketType::from(matrix);
    // println!("{:?}", matrix[0].to_vec());
    // println!("{:?}", matrix[1].to_vec());

    writer.write_image(&mut matrix).unwrap();
}
