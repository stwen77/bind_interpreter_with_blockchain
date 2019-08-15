extern crate rhai;

use rhai::Engine;

#[test]
fn test_unary_minus() {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<i64>("let x = -5; x").unwrap(), -5);

    assert_eq!(engine.eval::<i64>("fn n(x) { -x } n(5)").unwrap(), -5);

    assert_eq!(engine.eval::<i64>("5 - -(-5)").unwrap(), 0);
}
