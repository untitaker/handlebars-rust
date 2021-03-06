use helpers::{HelperDef};
use registry::{Registry};
use context::{Context, JsonRender};
use render::{Renderable, RenderContext, RenderError, render_error, Helper};

#[derive(Clone, Copy)]
pub struct LookupHelper;

impl HelperDef for LookupHelper {
    fn call(&self, c: &Context, h: &Helper, _: &Registry, rc: &mut RenderContext) -> Result<(), RenderError> {
        let value_param = try!(h.param(0).ok_or_else(|| render_error("Param not found for helper \"lookup\"")));
        let index_param = try!(h.param(1).ok_or_else(|| render_error("Insuffitient params for helper \"lookup\"")));

        let partial_path: String = if index_param.starts_with("@") {
            rc.get_local_var(index_param).render()
        } else {
            index_param.to_owned()
        };
        let lookup_path = format!("{}.[{}]", value_param, partial_path);
        let value = c.navigate(rc.get_path(), &lookup_path);
        let r = value.render();
        try!(rc.writer.write(r.into_bytes().as_ref()));
        Ok(())
    }
}

pub static LOOKUP_HELPER: LookupHelper = LookupHelper;

#[cfg(test)]
mod test {
    use template::{Template};
    use registry::{Registry};

    use std::collections::BTreeMap;

    #[test]
    fn test_lookup() {
        let t0 = Template::compile("{{#each v1}}{{lookup ../../v2 @index}}{{/each}}".to_string()).ok().unwrap();
        let t1 = Template::compile("{{#each v1}}{{lookup ../../v2 1}}{{/each}}".to_string()).ok().unwrap();
        let t2 = Template::compile("{{lookup kk \"a\"}}".to_string()).ok().unwrap();

        let mut handlebars = Registry::new();
        handlebars.register_template("t0", t0);
        handlebars.register_template("t1", t1);
        handlebars.register_template("t2", t2);

        let mut m :BTreeMap<String, Vec<u16>> = BTreeMap::new();
        m.insert("v1".to_string(), vec![1u16, 2u16, 3u16]);
        m.insert("v2".to_string(), vec![9u16, 8u16, 7u16]);

        let m2 = btreemap!{
            "kk".to_string() => btreemap!{"a".to_string() => "world".to_string()}
        };

        let r0 = handlebars.render("t0", &m);
        assert_eq!(r0.ok().unwrap(), "987".to_string());

        let r1 = handlebars.render("t1", &m);
        assert_eq!(r1.ok().unwrap(), "888".to_string());

        let r2 = handlebars.render("t2", &m2);
        assert_eq!(r2.ok().unwrap(), "world".to_string());
    }
}
