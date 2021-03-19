 use crate::vec3::Color;
 use std::io;
 use std::io::Write;


 pub fn write_color(stream: &mut impl Write, pixel_color: Color ) -> Result<(), io::Error> {
     
     let r = pixel_color.x;
     let g = pixel_color.y;
     let b = pixel_color.z;

    // write to file
     match stream.write_fmt(format_args!(
             "{} {} {}\n",
             (256.0 * r.clamp(0.0, 0.999)) as i32,
             (256.0 * g.clamp(0.0, 0.999)) as i32,
             (256.0 * b.clamp(0.0, 0.999)) as i32
     )) {
         Ok(_) => Ok(()),
         Err(e) => Err(e),
     }
 }
