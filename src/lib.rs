#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Units {
    Miles,
    Kilometers,
}

pub fn distance(start: Location, end: Location, units: Units) -> f64 {
    const KILOMETERS: f64 = 6371.0;
    const MILES: f64 = 3960.0;
    let r: f64;

    match units {
        Units::Miles => r = MILES,
        Units::Kilometers => r = KILOMETERS
    }

    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin()) + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    r * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haversine_distance_in_miles() {
        assert_eq!(0.341336828310639, distance(Location { latitude: 38.898556, longitude: -77.037852 }, Location { latitude: 38.897147, longitude: -77.043934 }, Units::Miles));
    }

    #[test]
    fn haversine_distance_in_kilometers() {
        assert_eq!(0.5491557912038084, distance(Location { latitude: 38.898556, longitude: -77.037852 }, Location { latitude: 38.897147, longitude: -77.043934 }, Units::Kilometers));
    }
}
