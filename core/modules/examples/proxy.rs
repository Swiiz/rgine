use rgine_modules::prelude::*;

fn main() {
    Engine::new::<Example>().run_with(StartEvent);
}

pub struct CountUpEvent(u32);

pub struct Example;
impl Module for Example {
    type ListeningTo = (CountUpEvent,);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        let mut counter = 0;

        ctx.add_event_proxy(move |event, queue| {
            let _event = &event.inner;
            let _metadata = &event.metadata;

            if counter < 10 {
                counter += 1;
                queue.push(CountUpEvent(counter));
            }
        });

        Ok(Self)
    }
}

impl Listener<CountUpEvent> for Example {
    fn on_event(&mut self, event: &mut CountUpEvent, _: &mut EventQueue) {
        println!("Current count: {}", event.0)
    }
}
