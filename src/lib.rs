#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![feature(test)]
extern crate test;

#[test]
fn haversine_distance_in_miles() {
    assert_eq!(0.341336828310639, distance(Location{latitude: 38.898556,longitude: -77.037852},  Location{latitude: 38.897147, longitude: -77.043934}, Units::Miles));
}

#[test]
fn haversine_distance_in_kilometers() {
    assert_eq!(0.5491557912038084, distance(Location{latitude: 38.898556,longitude: -77.037852},  Location{latitude: 38.897147, longitude: -77.043934}, Units::Kilometers));
}


pub struct Location {
    latitude:f64,
    longitude:f64
}

pub enum Units {
    Miles, 
    Kilometers
}

pub fn distance(start:Location, end:Location, units: Units) -> f64 {
    let kilometers: f64 = 6371.0;
    let miles: f64 = 3960.0;
    let mut r: f64 = 0.0;

    match units {
        Units::Miles => r = miles,
        Units::Kilometers => r = kilometers
                                                }

    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat/2.0).sin()) * ((d_lat/2.0).sin()) + ((d_lon/2.0).sin()) * ((d_lon/2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0-a).sqrt()));

    return r * c;
}
