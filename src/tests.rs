#[cfg(test)]
mod tests {
    use std::{sync::RwLock, thread::sleep, time::Duration};

    use crate::{self as regio, Component};

    #[regio::component]
    struct A {}

    impl Component for A {
        fn new() -> Self
        where
            Self: Sized,
        {
            A {}
        }

        fn init(&self) {
            println!("Init A")
        }
    }

    impl A {
        pub fn name(&self) -> &'static str {
            "A"
        }
    }

    #[regio::component]
    struct B {
        name: RwLock<Option<String>>,
    }

    impl Component for B {
        fn new() -> Self
        where
            Self: Sized,
        {
            B {
                name: RwLock::new(None),
            }
        }

        fn init(&self) {
            self.name.write().unwrap().replace("B".to_string());
            println!("Init B")
        }
    }

    impl B {
        pub fn set_name(&self, name: String) {
            self.name.write().unwrap().replace(name);
        }

        pub fn name(&self) -> String {
            self.name.read().unwrap().clone().unwrap()
        }
    }

    #[test]
    #[regio::init]
    fn it_works() {
        assert_eq!(regio::get::<A>().name(), "A");
        assert_eq!(regio::get::<B>().name(), "B");

        regio::get::<B>().set_name("B2".to_string());
        assert_eq!(regio::get::<B>().name(), "B2");

        #[regio::inject(a, A)]
        fn inject() {
            assert_eq!(regio::get::<A>().name(), "A");
        }
        inject()
    }
}
