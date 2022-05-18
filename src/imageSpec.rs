
// fn print_test_url(url: &str){
//     use std::borrow::Borrow;
//     let spec1 = Spec::new_resize(600, 800, resize::SampleFilter::CatmullRom);
//     let spec2 = Spec::new_watermark(20, 20);
//     let spec3 = Spec::new_filter(filter::FIlter::Marine);
//     let image_spec = ImageSpec::new(Vec![spec1, spec2, spec3]);
//     let s: String = image_spec.borrow().into();
//     let test_image = percent_encode(url.asbytes(), NON_ALPHANUMERIC).to_string();
//     println!("test url:http://localhost:3000/image/{}/{}",s,test_image);
// }