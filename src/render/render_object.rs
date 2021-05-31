use femtovg::renderer::OpenGl;
use femtovg::Canvas;

trait RenderObj {
    fn render(&mut self, canvas: &mut Canvas<OpenGl>);
}
