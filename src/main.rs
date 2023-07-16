use rand::Rng;
use std::num::ParseFloatError;

mod consts;
use consts::EARTH_RADIUS_KM;

fn main() {
    let berlin = Point {
        lat: 52.523403,
        lon: 13.411400,
    };
    let paris = Point {
        lat: 48.856667,
        lon: 2.350987,
    };

    println!(
        "the distance between paris and berlin is {:?} km",
        paris.distance(&berlin)
    );
    println!("paris json :\n  {}", paris.to_json());
    println!(
        "random Point: {:?}",
        Point::new_random().validate()
    );
    println!("midpoint: {:?}", paris.midpoint(&berlin).to_degrees());
    println!(
        "is paris within radius of 100km: {:?}",
        paris.is_within_radius(&berlin, 1410.)
    );
    println!(
        "new from string: {:?}",
        Point::new_from_string(
            "48° 51' 23.796''".to_string(),
            "48° 51' 23.796''".to_string()
        )
        .unwrap()
    );
    println!("offset: {:?}", paris.offset(10000.1, 180.).distance(&paris));
}
#[derive(Debug)]
struct Point {
    lat: f64,
    lon: f64,
}
impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        let radius: f64 = EARTH_RADIUS_KM;

        let (lat1, lon1) = self.rad();

        let (lat2, lon2) = other.rad();

        let diff_lat: f64 = (lat1 - lat2) / 2.;
        let diff_lon = (lon1 - lon2) / 2.;

        let a: f64 = diff_lat.sin().powi(2) + diff_lon.sin().powi(2) * lat1.cos() * lat2.cos();
        let c: f64 = 2. * a.sqrt().atan2((1. - a).sqrt());

        (radius * c) as f64
    }

    pub fn new(lat: f64, lon: f64) -> Point {
        Point { lat, lon }
    }

    pub fn to_degrees(&self) -> (f64, f64) {
        (self.lat, self.lon)
    }

    pub fn new_from_string(lon: String, lat: String) -> Result<Point, ParseFloatError> {
        let lon: Vec<&str> = lon.split_whitespace().collect::<Vec<&str>>();
        let lat: Vec<&str> = lat.split_whitespace().collect::<Vec<&str>>();

        let lon_deg: f64 = lon[0][0..(lon[0].len() - 2)].parse()?;
        let lon_min: f64 = lon[1][0..(lon[1].len() - 1)].parse()?;
        let lon_sec: f64 = lon[2][0..(lon[2].len() - 2)].parse()?;

        let lat_deg: f64 = lat[0][0..(lat[0].len() - 2)].parse()?;
        let lat_min: f64 = lat[1][0..(lat[1].len() - 1)].parse()?;
        let lat_sec: f64 = lat[2][0..(lat[2].len() - 2)].parse()?;

        let lon = lon_deg + lon_min / 60. + lon_sec / 3600.;
        let lat = lat_deg + lat_min / 60. + lat_sec / 3600.;

        Ok(Point::new(lon, lat))
    }

    pub fn is_within_radius(&self, other: &Self, radius_in_km: f64) -> bool {
        let distance = self.distance(other);

        distance <= radius_in_km
    }

    pub fn validate(&self) -> bool {
        let lat = self.lat;
        let lon = self.lon;

        let lat_valid = lat >= -90. && lat <= 90.;
        let lon_valid = lon >= -180. && lon <= 180.;

        lat_valid && lon_valid
    }

    pub fn new_random() -> Point {
        let mut rng = rand::thread_rng();

        let lat = rng.gen_range((-90.)..(90.));
        let lon = rng.gen_range((-180.)..(180.));

        Point { lat, lon }
    }

    pub fn midpoint(&self, other: &Self) -> Point {
        let (lat1, lon1) = self.rad();
        let (lat2, lon2) = other.rad();

        let x: f64 = (lat1.cos() * lon1.cos() + lat2.cos() * lon2.cos()) / 2.0;
        let y: f64 = (lat1.cos() * lon1.sin() + lat2.cos() * lon2.sin()) / 2.0;
        let z: f64 = (lat1.sin() + lat2.sin()) / 2.0;

        Point {
            lat: (z.atan2((x * x + y * y).sqrt())).to_degrees(),
            lon: (y.atan2(x)).to_degrees(),
        }
    }

    pub fn offset(&self, distance: f64, bearing: f64) -> Point {
        let radius: f64 = EARTH_RADIUS_KM;
        let bearing = bearing.to_radians();
        let (lat1, lon1) = self.rad();
        let rad = distance / radius;

        let lat = (lat1.sin() * rad.cos() + lat1.cos() * rad.sin() * bearing.cos()).asin();
        let lng_pre_1 = { bearing.sin() * rad.sin() * lat1.cos() };
        let lng = lng_pre_1.atan2(rad.cos() - lat1.sin() * lat.sin()) + lon1;

        println!("lat2: {}, lon2: {}", lat1.to_degrees(), lon1.to_degrees());

        Point {
            lat: lat.to_degrees(),
            lon: lng.to_degrees(),
        }
    }

    pub fn to_json(&self) -> String {
        format!("{{\"lat\":{}, \"lon\":{}}}", self.lat, self.lon)
    }

    fn rad(&self) -> (f64, f64) {
        (self.lat.to_radians(), self.lon.to_radians())
    }
}
