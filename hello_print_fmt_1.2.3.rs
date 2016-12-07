use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入到一个缓冲区
        // 中（第一个除数f）
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // `f` 是一个缓冲区（buffer），此方法必须将格式化的字符串写入其中
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {        
        // `write!` 和 `format!` 类似，但它会将格式化后的字符串写入到一个缓冲区
        // 中（第一个除数f）
        write!(f, "RGB {red:03},{green:03},{blue:03} 0x{red:02.X}{red:02.X}{red:02.x} ",
               red=self.red,green=self.green, blue=self.blue)
    }
}


fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 1 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 一旦添加了针对 fmt::Display 的实现，则要用 {} 对输出内容进行转换
        println!("{}", *color)
    }

}


