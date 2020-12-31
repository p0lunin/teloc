use teloc::inject;

#[derive(Debug, PartialEq)]
struct City(String);

struct WeatherService<'a> {
    _city: &'a City,
}

#[inject]
impl<'a> WeatherService<'a> {
    fn new(_city: &'a City, _foo: i32) -> Self {
        Self { _city }
    }
}

#[test]
fn test() {}
