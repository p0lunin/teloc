use teloc::{inject, scopei, Resolver, ServiceProvider};

#[derive(Debug, PartialEq)]
struct City(String);

struct WeatherService<'a> {
    city: &'a City,
}

#[inject]
impl<'a> WeatherService<'a> {
    fn new(city: &'a City) -> Self {
        Self { city }
    }
}

#[test]
fn test() {
    let sp = ServiceProvider::new()
        .add_scoped_i::<City>()
        .add_scoped::<WeatherService>();

    let scope = sp.scope(scopei![City("Odessa".into()),]);

    let s1: &WeatherService = scope.resolve();
    let s2: &WeatherService = scope.resolve();

    assert_eq!(s1.city.0, "Odessa".to_string());
    assert_eq!(s1.city, s2.city);
}
