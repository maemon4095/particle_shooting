use fixed_vector::fixed_vector;

#[derive(Debug, PartialEq)]
#[fixed_vector(i32; x, y, z)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test_vector3_add() {
    debug_assert!(
        Vector3 { x: 1, y: 1, z: 1 } + Vector3 { x: 1, y: 2, z: 3 } == Vector3 { x: 2, y: 3, z: 4 }
    );
}

#[test]
fn test_vector3_sub() {
    debug_assert!(
        Vector3 { x: 1, y: 2, z: 3 } - Vector3 { x: 1, y: 1, z: 1 } == Vector3 { x: 0, y: 1, z: 2 }
    );
}

#[test]
fn test_vector3_mul() {
    debug_assert!(Vector3 { x: 1, y: 2, z: 3 } * 2 == Vector3 { x: 2, y: 4, z: 6 });
}

#[test]
fn test_vector3_div() {
    debug_assert!(Vector3 { x: 2, y: 4, z: 6 } / 2 == Vector3 { x: 1, y: 2, z: 3 });
}

#[fixed_vector(T; x, y, z)]
struct GenericVector3<T> {
    x: T,
    y: T,
    z: T,
}

#[fixed_vector(i32; 0, 1, 2)]
struct VectorTuple(i32, i32, i32);

#[fixed_vector(T; 0, 1, 2)]
struct GenericVectorTuple<T>(T, T, T);
