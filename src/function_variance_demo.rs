#[cfg(test)]
mod test {
    #[test]
    fn function_variance_demo() {
        trait FlyBehavior {
            fn fly(&self);
        }

        struct Bird {}
        impl FlyBehavior for Bird {
            fn fly(&self) {
                println!("Bird flies by flapping wings");
            }
        }

        fn fly(fly_behavior: impl FlyBehavior) -> String {
            fly_behavior.fly();
            "Flying".to_string()
        }

        fn bird_fly<T>(fly_fn: T)
        where
            T: FnOnce(Bird) -> String {
            let bird = Bird {};
            println!("{}", fly_fn(bird));
        }

        bird_fly(fly);
    }
}
