use std::{rc::Rc, cell::RefCell};

use rstest::rstest;

use crate::{materials::{DiffuseMaterial, TraceFunction, Material}, imaging::color::Color, primitives::{PlaneXY, Decorator, Primitive}, math::{Ray, Vector, approx}, vc, pt};

#[rstest]
fn diffuse_lighting_on_plane_1() {
    let material = Rc::new(DiffuseMaterial::new(Color::white()));
    let plane = Rc::new(PlaneXY::new());
    let decorated_plane = Rc::new(Decorator::new(material, plane));
    let ray = Ray::new(pt!(0, 0, 1), vc!(0, 0, -1));
    let hit = decorated_plane.find_first_positive_hit(&ray).unwrap();
    let material = hit.material.as_ref();
    let direction = vc!(0, 0, 1);
    let last_trace_direction = Rc::new(RefCell::new(None));
    let trace: TraceFunction = {
        let copy = last_trace_direction.clone();

        Box::new(move |v: &Vector<3>, _w: f64| {
            *copy.borrow_mut() = Some(v.clone());
            Color::black()
        })
    };

    let mut material_color = material.at(&direction, trace);

    assert_eq!(None, *last_trace_direction.borrow());
    assert_eq!(Color::black(), material_color.current());

    let constant = 2.0f64.sqrt() / 2.0;

    material_color.refine();
    assert_eq!(approx(vc!(0, 0, 1)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(0.5, -constant, 0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(-0.5, -constant, 0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(0.5, constant, 0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(-0.5, constant, 0.5)), last_trace_direction.borrow().unwrap());
}

#[rstest]
fn diffuse_lighting_on_plane_2() {
    let material = Rc::new(DiffuseMaterial::new(Color::white()));
    let plane = Rc::new(PlaneXY::new());
    let decorated_plane = Rc::new(Decorator::new(material, plane));
    let ray = Ray::new(pt!(0, 0, -1), vc!(0, 0, 1));
    let hit = decorated_plane.find_first_positive_hit(&ray).unwrap();
    let material = hit.material.as_ref();
    let direction = vc!(0, 0, -1);
    let last_trace_direction = Rc::new(RefCell::new(None));
    let trace: TraceFunction = {
        let copy = last_trace_direction.clone();

        Box::new(move |v: &Vector<3>, _w: f64| {
            *copy.borrow_mut() = Some(v.clone());
            Color::black()
        })
    };

    let mut material_color = material.at(&direction, trace);

    assert_eq!(None, *last_trace_direction.borrow());
    assert_eq!(Color::black(), material_color.current());

    let constant = 2.0f64.sqrt() / 2.0;

    material_color.refine();
    assert_eq!(approx(vc!(0, 0, -1)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(0.5, -constant, -0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(-0.5, -constant, -0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(0.5, constant, -0.5)), last_trace_direction.borrow().unwrap());

    material_color.refine();
    assert_eq!(approx(vc!(-0.5, constant, -0.5)), last_trace_direction.borrow().unwrap());
}
