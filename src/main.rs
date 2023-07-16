use rand::Rng;
use std::num::ParseFloatError;

fn main() {
    let berlin = Coordinates {
        lat: 52.523403,
        lon: 13.411400,
    };
    let paris = Coordinates {
        lat: 48.856667,
        lon: 2.350987,
    };

    println!(
        "the distance between paris and berlin is {:?} km",
        paris.distance(&berlin)
    );
    println!("paris json :\n  {}", paris.to_json());
    println!(
        "random coordinates: {:?}",
        Coordinates::new_random().validate()
    );
    println!("midpoint: {:?}", paris.midpoint(&berlin).to_degrees());
    println!(
        "is paris within radius of 100km: {:?}",
        paris.is_within_radius(&berlin, 100.)
    );
    println!(
        "new from string: {:?}",
        Coordinates::new_from_string(
            "48° 51' 23.796''".to_string(),
            "48° 51' 23.796''".to_string()
        )
        .unwrap()
    );
    println!(
        "offset: {:?}",
        paris.offset(9., 0.).distance(&paris)
    );
}
#[derive(Debug)]
struct Coordinates {
    lat: f64,
    lon: f64,
}
impl Coordinates {

    pub fn distance(&self, other: &Self) -> f64 {
        let radius: f64 = 6_371.;

        let (lat1, lon1) = self.rad(); 

        let (lat2, lon2) = other.rad();

        let diff_lat: f64 = (lat1 - lat2) / 2.;
        let diff_lon = (lon1 - lon2) / 2.;

        // haversine formula

        let a = diff_lat.sin().powi(2) + diff_lon.sin().powi(2) * lat1.cos() * lat2.cos();
        let c = 2. * a.sqrt().atan2((1. - a).sqrt());

        (radius * c) as f64
    }

    pub fn new(lat: f64, lon: f64) -> Coordinates {
        Coordinates { lat, lon }
    }

    pub fn to_degrees(&self) -> (f64, f64) {
        (self.lat, self.lon)
    }

    pub fn new_from_string(lon: String, lat: String) -> Result<Coordinates, ParseFloatError> {
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

        Ok(Coordinates::new(lon, lat))
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

    pub fn new_random() -> Coordinates {
        let mut rng = rand::thread_rng();

        let lat = rng.gen_range((-90.)..(90.));
        let lon = rng.gen_range((-180.)..(180.));

        Coordinates { lat, lon }
    }

    pub fn midpoint(&self, other: &Self) -> Coordinates {
        let (lat1, lon1) = self.rad();

        let (lat2, lon2) = other.rad();

        let x = (lat1.cos() * lon1.cos() + lat2.cos() * lon2.cos()) / 2.0;
        let y = (lat1.cos() * lon1.sin() + lat2.cos() * lon2.sin()) / 2.0;
        let z = (lat1.sin() + lat2.sin()) / 2.0;

        Coordinates {
            lat: (z.atan2((x * x + y * y).sqrt())).to_degrees(),
            lon: (y.atan2(x)).to_degrees(),
        }
    }
    
    pub fn offset(&self, distance: f64, bearing: f64) -> Coordinates {
        let radius: f64 = 6_371.;
        let bearing = bearing.to_radians();
        let (lat1, lon1) = self.rad();
        let angular_distance = distance / radius;
        
        let lat_res = lat1.sin() * angular_distance.cos() + lat1.cos() * angular_distance.sin() * bearing.cos();
        let lat2 = lat_res.asin();
        let lon_res = lon1 + (bearing.sin() * angular_distance.sin() * lat1.cos()) / (lat2.cos());
        let lon2 = lon_res;

        println!("lat2: {}, lon2: {}", lat2.to_degrees(), lon2.to_degrees());
        
        Coordinates {
            lat: lat2.to_degrees(),
            lon: lon2.to_degrees(),
        }
    }
    
    pub fn to_json(&self) -> String {
        format!("{{\"lat\":{}, \"lon\":{}}}", self.lat, self.lon)
    }

    fn rad(&self) -> (f64, f64){
            ( self.lat.to_radians(), self.lon.to_radians(),)
    }
}
