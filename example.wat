(module  (func (export "_start") (result i32) (local $total i32) (local $count i32) (local.set $total (i32.const 0)) (local.set $count (i32.const 0)) (loop $while_start (br_if $while_start (i32.lt_s (local.get $count) (i32.const 10))) (local.set $count (i32.add (local.get $count) (i32.const 1))) (local.set $total (i32.add (local.get $total) (local.get $count)))) (local.get $total)))