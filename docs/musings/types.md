1.0
2.0

(option) => nil
(option true) => true

(def = [option option]
  // if arg1 is null and arg2 is null then true
  // else if arg1 is null or arg2 is null then false
  // else invoke = for option and option

true
false
(bool) => false
(bool true) => true
(string true) => "true"
(= true true) => true

:x
(type :x) => :symbol

(int32) => 0
(type (int32)) => :int32
(div (int32 1)) => 1
(div (int32 "1.2")) => 1
(float (int32)) => 0.0
(mul 2 2.0) => 4.0

[0 0]
(array 0 0) => [0 0]
(type [0 0]) => :array
(first [0 0]) => 0
(type (first [0 0])) => :int32
(append [0] 1) => [0 1]
(def append [array item]
  )

(set 0 1 1) => |0 1|
(contains? |0 1| 2) => false
(contains? |0 1| 2) => true
(def contains? [set item]
  (any? set #(= %1 item)))


(ordered-set 2 1 0 1) => /0 1 2/
(add [set item]
  (

{:x 1}
(type {:x 1}) => :map

"string"
(type "string") => :string
(int32 "string") => nil

(div (float 1.0) (float 2.0)) => 0.5
(div 1 2) => 0

(type (def...)) => :function


(def fibonacci [(= 0 number)] 0)
(def fibonacci [(= 1 number)] 1)
(def fibonacci [number] ^(- 1 number) ^(- 2 number))

