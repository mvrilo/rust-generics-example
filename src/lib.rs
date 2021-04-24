pub trait Caller {
    fn call(self);
}

#[derive(Default, Clone)]
struct CallerImpl;

impl Caller for CallerImpl {
    fn call(self) {}
}

#[derive(Clone)]
pub struct MyService<'a, C: Caller + Clone> {
    pub name: &'a str,
    pub caller: &'a C,
}

impl<'a, C: Caller + Clone> MyService<'a, C> {
    pub fn new(name: &'a str, caller: &'a C) -> MyService<'a, C> {
        MyService { name, caller }
    }

    pub fn call(&self) {
        self.caller.clone().call();
    }
}

#[allow(dead_code)]
fn new() {
    MyService::new("name", &CallerImpl::default()).call();
}
