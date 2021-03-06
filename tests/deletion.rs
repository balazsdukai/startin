use rand::prelude::*;
use startin;

#[test]
fn simple() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![1.1, 1.07, 12.5]);
    pts.push(vec![11.0, 1.02, 7.65]);
    pts.push(vec![11.05, 11.1, 33.0]);
    pts.push(vec![1.0, 11.0, 21.0]);
    pts.push(vec![9.0, 5.0, 21.0]);
    pts.push(vec![12.0, 5.1, 21.0]);
    pts.push(vec![8.0, 8.0, 21.0]);
    pts.push(vec![12.0, 8.1, 21.0]);
    pts.push(vec![4.0, 5.15, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    let _re = dt.remove(7);
    assert_eq!(8, dt.number_of_vertices());
    assert_eq!(8, dt.number_of_triangles());
}

#[test]
fn insert_delete() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    let mut rng = rand::thread_rng();
    let size = 10.0_f64;
    for _i in 0..100 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        pts.push(vec![x * size, y * size, 2.0]);
    }
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    let _re = dt.insert_one_pt(3.05, 3.1, 33.0);
    let re = dt.remove(dt.number_of_vertices() - 1);
    assert_eq!(true, re.is_ok());
    assert_eq!(100, dt.number_of_vertices());
    assert!(dt.is_valid());
}

#[test]
fn insert_delete_them_many() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    let mut rng = rand::thread_rng();
    let size = 10.0_f64;
    for _i in 0..10 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        pts.push(vec![x * size, y * size, 2.0]);
    }
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    assert_eq!(10, dt.number_of_vertices());
    for i in 5..10 {
        let _re = dt.remove(i);
    }
    assert_eq!(5, dt.number_of_vertices());
    // assert_eq!(0, dt.number_of_triangles());

    assert!(dt.is_valid());
}

#[test]
fn collinear() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![1.0, 0.0, 7.65]);
    pts.push(vec![2.0, 0.0, 33.0]);
    pts.push(vec![3.0, 0.0, 33.0]);
    pts.push(vec![4.0, 0.0, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);
    assert_eq!(5, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
    let _re = dt.insert_one_pt(3.0, 3.0, 33.0);
    assert_eq!(6, dt.number_of_vertices());
    assert_eq!(4, dt.number_of_triangles());
}

#[test]
fn convexhull() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![1.0, 0.0, 7.65]);
    pts.push(vec![2.0, 2.0, 33.0]);
    pts.push(vec![0.0, 2.0, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);
    let _re = dt.remove(3);
    assert_eq!(3, dt.number_of_vertices());
    assert_eq!(1, dt.number_of_triangles());
    let _re = dt.remove(2);
    assert_eq!(2, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
}

#[test]
fn cocircular() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![1.0, 0.0, 7.65]);
    pts.push(vec![1.0, 1.0, 33.0]);
    pts.push(vec![0.0, 1.0, 21.0]);
    let y: f64 = 0.5 + ((0.5 * 0.5 + 0.5 * 0.5) as f64).sqrt();
    pts.push(vec![0.5, y, 21.0]);
    pts.push(vec![0.5, 0.5, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    let _re = dt.remove(6);
    assert_eq!(5, dt.number_of_vertices());
    assert_eq!(3, dt.number_of_triangles());
}

#[test]
fn deletion_impossible() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![1.1, 1.07, 12.5]);
    pts.push(vec![11.0, 1.02, 7.65]);
    pts.push(vec![11.05, 11.1, 33.0]);
    pts.push(vec![1.0, 11.0, 21.0]);
    pts.push(vec![9.0, 5.0, 21.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);
    assert_eq!(Err("Cannot remove the infinite vertex"), dt.remove(0));
    assert_eq!(Err("Vertex does not exist"), dt.remove(7));
    let _re = dt.remove(5);
    assert_eq!(Err("Vertex does not exist"), dt.remove(5));
}

#[test]
fn grid() {
    let mut dt = startin::Triangulation::new();
    for i in 0..10 {
        for j in 0..10 {
            let _re = dt.insert_one_pt(i as f64, j as f64, 1.0);
        }
    }
    let _re = dt.remove(1);
    let _re = dt.remove(10);
    let _re = dt.remove(77);
    assert!(dt.is_valid());
}

#[test]
fn simple_grid() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![1.0, 0.0, 7.65]);
    pts.push(vec![1.0, 1.0, 33.0]);
    pts.push(vec![0.0, 1.0, 21.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    let _re = dt.remove(1);
    let _re = dt.remove(3);
    assert_eq!(2, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
}
