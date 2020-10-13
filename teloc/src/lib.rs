pub use teloc_macros::{Teloc, container};

pub trait Get<T> {
    fn get(&mut self) -> T;
}
pub trait GetClone<T: Clone> {
    fn get_clone(&self) -> T;
}

#[cfg(test)]
mod tests {
    use teloc_macros::{Teloc, container};
    use super::{Get, GetClone};
    use std::rc::Rc;

    struct ConstService {
        data: i32,
        data2: u8,
    }
    impl ConstService {
        pub fn new(data: i32, data2: u8) -> Self {
            ConstService { data, data2 }
        }
    }
    #[derive(Teloc)]
    struct ControllerA {
        #[new(0, 1)]
        service: ConstService,
    }
    #[derive(Teloc)]
    struct ControllerB {
        #[new(1, 5)]
        service: ConstService,
    }
    #[derive(Teloc)]
    struct Schema {
        a: ControllerA,
        b: ControllerB,
    }
    #[test]
    fn test() {
        let mut container = container! [
            ControllerA,
            ControllerB,
            Schema
        ];
        let schema: Schema = container.get();
        assert_eq!(schema.a.service.data, 0);
        assert_eq!(schema.a.service.data2, 1);
        assert_eq!(schema.b.service.data, 1);
        assert_eq!(schema.b.service.data2, 5);
    }
}
