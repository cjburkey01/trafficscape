use cgmath::Vector2;

type Vec2 = Vector2<f64>;

#[derive(Debug, Clone)]
pub struct Transform {
    pos: Vec2,
    rot: f64,
}

pub trait TransformImpl {
    fn transform(&self) -> &Transform;
}

#[derive(Debug, Clone)]
pub struct PhysObj {
    mass: f64,
    velocity: Vec2,
}

pub trait PhysObjImpl {
    fn phys_obj(&self) -> &PhysObj;
}

#[derive(Debug, Clone)]
pub struct Car {
    transform: Transform,
    phys_obj: PhysObj,
}

impl TransformImpl for Car {
    fn transform(&self) -> &Transform {
        &self.transform
    }
}

impl PhysObjImpl for Car {
    fn phys_obj(&self) -> &PhysObj {
        &self.phys_obj
    }
}
