(module (func $fact (param $n i32) (result i32) (if (result i32) (i32.eq (local.get $n) (i32.const 0)) (then (i32.const 1)) (else (i32.mul (call $fact (i32.sub (local.get $n) (i32.const 1))) (local.get $n))))) (func (export "_start") (result i32)  (local $x i32) (local.set $x (i32.add (i32.const 1) (i32.const 2))) (call $fact (local.get $x))))