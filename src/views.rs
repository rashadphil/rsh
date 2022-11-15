pub mod baseview;
pub mod table;

pub trait RenderView {
    fn render(&self) -> Vec<String>;
}

