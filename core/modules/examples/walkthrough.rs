use rgine_logger::info;
use rgine_modules::prelude::*;

pub struct OnPrint {
    pub message: String,
}

fn main() {
    Engine::new::<AutoLog>().run_with(StartEvent);
}

pub struct AutoLog {
    lang: Dependency<Language>,
}
impl Module for AutoLog {
    type ListeningTo = (StartEvent,);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        ctx.dependency::<Printer>()?;

        Ok(Self {
            lang: ctx.dependency::<Language>()?,
        })
    }
}
impl Listener<StartEvent> for AutoLog {
    fn on_event(&mut self, _: &mut StartEvent, queue: &mut EventQueue) {
        let lang = self.lang.read_state();

        queue.push(OnPrint {
            message: lang.on_start.clone(),
        })
    }
}

pub struct Language {
    on_start: String,
}
impl Module for Language {
    type ListeningTo = ();
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(Self {
            on_start: "Hello world".to_owned(),
        })
    }
}

pub struct Printer {}
impl Module for Printer {
    type ListeningTo = (OnPrint,);
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(Self {})
    }
}
impl Listener<OnPrint> for Printer {
    fn on_event(&mut self, event: &mut OnPrint, _: &mut EventQueue) {
        info!("{}", event.message)
    }
}
