use crate::model::Page;

const JS_LIBRARIES_TEMPLATE: &str = include_str!("../templates/js-libraries.template");
const PAGE_TEMPLATE: &str = include_str!("../templates/page.template");

pub(crate) fn load<'a>() -> TemplateEngine<'a> {
    let mut engine = upon::Engine::new();
    engine.add_template("page", PAGE_TEMPLATE).unwrap();
    engine
        .add_template("js-libraries", JS_LIBRARIES_TEMPLATE)
        .unwrap();
    TemplateEngine { engine }
}

pub(crate) struct TemplateEngine<'a> {
    engine: upon::Engine<'a>,
}

impl<'a> TemplateEngine<'a> {
    pub(crate) fn render(&self, target: std::fs::File, page: Page) {
        let template = self.engine.get_template("page").unwrap();
        template.render_to_writer(target, page).unwrap();
    }
}
