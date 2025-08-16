;;
;;   HEAD
;;

;; HEAD -> MODULE
(module


;;
;;  INCLUDE
;;

;; INCLUDE -> NUMBER
(;@include "number/add.wat";)

;; INCLUDE -> SYSTEM
(;@include "system/exit.wat";)
(;@include "system/write.wat";)


;;
;;  BOTTOM
;;

;; BOTTOM -> MARK
)