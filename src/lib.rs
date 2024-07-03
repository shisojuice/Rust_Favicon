use std::io::{Cursor};
use image::{DynamicImage, ImageBuffer, Rgba};
use image::imageops::FilterType;
use ico;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::{Blob,BlobPropertyBag, Url};
use web_sys::js_sys::{Array, Uint8Array};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn favicon_generate(input_color: String,image_data: &[u8]) {
    let mut img = resize_image(&input_color,image::load_from_memory(image_data).unwrap());
    if input_color == "transparent" || input_color == "black" || input_color == "white"{
        let rgb_img = img.to_rgb8();
        // 左上のピクセルを背景色とする
        let background_color = rgb_img.get_pixel(0, 0);
        let mut output_img = ImageBuffer::new(rgb_img.width(), rgb_img.height());
        for (y, row) in rgb_img.rows().enumerate() {
            for (x, pixel) in row.enumerate() {
                if pixel == background_color {
                    if input_color == "transparent"{
                        output_img.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
                    }
                    if input_color == "black" {
                        output_img.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 255]));
                    }
                    if input_color == "white"{
                        output_img.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255]));
                    }
                } else {
                    output_img.put_pixel(x as u32, y as u32, Rgba([pixel[0], pixel[1], pixel[2], 255]));
                }
            }
        }
        img = DynamicImage::ImageRgba8(output_img);
    }

    // ICOエンコーダーを作成
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    // 各アイコンサイズに対してループ処理
    let icon_sizes = vec![16,32,48,96,144,192,240,288];
    for size in icon_sizes {
        let resized_img = img.resize(size, size, FilterType::Lanczos3);
        let rgba = resized_img.to_rgba8().to_vec();
        let icon_img = ico::IconImage::from_rgba_data(size, size, rgba);

        // エンコードした画像をICOディレクトリに追加
        let icon = ico::IconDirEntry::encode(&icon_img).unwrap();
        icon_dir.add_entry(icon);
    }
    let mut buffer = Cursor::new(Vec::new());
    icon_dir.write(&mut buffer).unwrap();
    let icon_data = buffer.into_inner();

    //DL
    let window = web_sys::window().unwrap();
    let uint8_array = Uint8Array::from(icon_data.as_slice());
    let parts = Array::new();
    parts.push(&uint8_array);
    // Blobを作成
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts,BlobPropertyBag::new().type_("image/vnd.microsoft.icon")).unwrap();
    // BlobのURLを取得
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    // a要素を作成
    let link = window.document().unwrap().create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
    link.set_href(&url);
    link.set_download("favicon.ico");
    link.click();
    // URLを解放
    Url::revoke_object_url(&url).unwrap();
}

// 画像のアスペクト比・操作しやすいサイズに設定・調整
fn resize_image(input_color: &String,img: DynamicImage) -> DynamicImage {
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // 横長の画像の場合
        (320, (320.0 / aspect_ratio) as u32)
    } else {
        // 縦長の画像の場合
        ((320.0 * aspect_ratio) as u32, 320)
    };
    // リサイズ
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    // 320x320のキャンバスを作成
    let mut canvas = DynamicImage::new_rgba8(320, 320);

    if input_color != "nothing" {
        // 左上のピクセルを背景色とする
        let rgb_img = resized_img.to_rgb8();
        let background_color = rgb_img.get_pixel(0, 0);
        let bkg_image= ImageBuffer::from_pixel(320, 320, Rgba([background_color[0], background_color[1], background_color[2], 255]));
        image::imageops::overlay(&mut canvas, &bkg_image, 0, 0);
    }
    // リサイズ画像をキャンバスの中央に貼付
    let x = (320 - resized_img.width()) / 2;
    let y = (320 - resized_img.height()) / 2;
    image::imageops::overlay(&mut canvas, &resized_img, x.into(), y.into());
    canvas
}

#[wasm_bindgen]
pub fn favicon_check(image_data: &[u8]) -> String {
    let icon_dir = ico::IconDir::read(Cursor::new(image_data)).unwrap();
    let mut result = String::new();
    result.push_str("出力結果 \n");

    for entry in icon_dir.entries() {
        result.push_str("Width : ");
        result.push_str(&*entry.width().to_string());
        result.push_str(" x Height : ");
        result.push_str(&*entry.height().to_string());
        result.push_str("\n");
    }
    result
}
