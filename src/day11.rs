use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use rocket::fs::{NamedFile};
use rocket::{Data, get, post};
use rocket::http::ContentType;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};


#[get("/<path..>")]
pub async fn serve(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new("").join(path);
    if path.is_dir() {
        path.push("index.html");
    }
    NamedFile::open(path).await.ok()
}


#[post("/red_pixels", data="<data>")]
pub async fn red_pixels(data: Data<'_>, content_type: &ContentType) -> Option<String> {
    use image::{GenericImageView, ImageFormat};
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("image"),
    ]);
    let mut form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(r) => r,
        Err(_) => {
            return None
        }
    };

    if let Some(mut media) = form_data.files.remove("image") {
        let file = media.remove(0);
        let image = image::load(BufReader::new(File::open(file.path).expect("Cannot open file")), ImageFormat::Png).expect("Cannot load image");
        Some(format!("{}",image.pixels().filter(|(_,_,p)| {
            p.0[0] as u32 > (p.0[1] as u32 + p.0[2] as u32)
        }).count() as u32))
    } else {
        None
    }
}