; This should return false if it overflows...
; Signed bvadd overflow check function

; Check for overflow of a signed bit vector addition
(define-fun bvsaddo ((x (_ BitVec 32)) (y (_ BitVec 32))) Bool
    ; If result >= 0 and (x < 0) and (y < 0)
    (if (or (and (bvsge (bvadd x y) (_ bv0 32))
                 (bvslt x (_ bv0 32))
                 (bvslt y (_ bv0 32)))
            ; If result <= 0 and x > 0 and y > 0
            (and (bvsle (bvadd x y) (_ bv0 32))
                 (bvsgt x (_ bv0 32))
                 (bvsgt y (_ bv0 32))))
        false  ; Then there was an overflow
        true)) ; Else return true

(assert (bvsaddo #x00000000 #x7fffffff))
(assert (bvsaddo (_ bv0 32) (_ bv2147483647 32)))
(assert (not (bvsaddo #x00000001 #x7fffffff)))
(assert (not (bvsaddo (_ bv1 32) (_ bv2147483647 32))))

;-------------------------------------------------------------------------------

;Check for overflow of an unsigned bit vector addition
(define-fun bvuaddo ((x (_ BitVec 32)) (y (_ BitVec 32))) Bool
    ; If result < x or result < y
    (if (or (bvult (bvadd x y) x)
            (bvult (bvadd x y) y))
        false ; Then there was overflow
        true))

(assert (bvuaddo #x00000000 #xffffffff))
(assert (bvuaddo (_ bv0 32) (_ bv4294967295 32)))
(assert (not (bvuaddo #x00000001 #xffffffff)))
(assert (not (bvuaddo (_ bv1 32) (_ bv4294967295 32))))

;-------------------------------------------------------------------------------

; Check overflow of signed bit vector subtraction
(define-fun bvssubo ((x (_ BitVec 32)) (y (_ BitVec 32))) Bool
    (if (or (and (bvsge (bvsub x y) (_ bv0 32))
                 (bvslt x (_ bv0 32))
                 (bvslt y (_ bv0 32)))
            (and (bvsle (bvsub x y) (_ bv0 32))
                 (bvsgt x (_ bv0 32))
                 (bvsgt y (_ bv0 32))))
        false  ; Then there was an overflow
        true)) ; Else return true

(assert (bvssubo #x00000000 #x7fffffff))
(assert (bvssubo (_ bv0 32) (_ bv2147483647 32)))
(assert (bvssubo (_ bv0 32) (_ bv2147483648 32)))
(assert (not (bvssubo #x00000001 #x7fffffff)))
(assert (not (bvssubo (_ bv1 32) (_ bv2147483647 32))))

;-------------------------------------------------------------------------------

;Check overflow of unsigned bit vector subtraction
(define-fun bvusubo ((x (_ BitVec 32)) (y (_ BitVec 32))) Bool
    ; If x < y
    (if (bvult x y)
        false ; Then there was overflow
        true))

(assert (bvusubo #x00000000 #x00000000))
(assert (bvusubo (_ bv0 32) (_ bv0 32)))
(assert (bvusubo #x00000001 #x00000000))
(assert (bvusubo (_ bv1 32) (_ bv0 32)))
(assert (bvusubo #x00000001 #x00000001))
(assert (bvusubo (_ bv1 32) (_ bv1 32)))
(assert (not (bvusubo #x00000000 #x00000001)))
(assert (not (bvusubo (_ bv0 32) (_ bv1 32))))

