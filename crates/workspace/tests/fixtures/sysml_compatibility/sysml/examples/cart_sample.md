# META
~~~ini
description=SysML Example (State Space Representation): CartSample
type=file
~~~
# SOURCE
~~~sysml
// State Space Representation cart example

package CartSample {
    private import StateSpaceRepresentation::*;
    part def Cart {
        attribute mass :> ISQ::mass;

        attribute def CartInput :> Input {
            attribute force :> ISQ::force;
        }

        attribute def CartOutput :> Output {
            attribute velocity :> ISQ::speed;
        }

        attribute def CartState :> StateSpace {
            attribute velocity :> ISQ::speed;
        }

        attribute def CartStateDerivative :> StateDerivative {
            ref :>> stateSpace : CartState;
            attribute accel :> ISQ::acceleration;
        }
    }

    part def Pusher {
        attribute def PusherOutput :> Output {
            attribute force :> ISQ::force;
        }
    }

    part context {
        part cart : Cart {
            action cartBehavior : ContinuousStateSpaceDynamics {
                in input : CartInput;
                out output : CartOutput;
                :>> stateSpace : CartState;

                calc :>> getDerivative { 
                	in input: CartInput; 
                	in stateSpace: CartState;
                    new CartStateDerivative(input.force / mass)
                }
                calc :>> getOutput {
                	in :>> stateSpace : CartState;
                    new CartOutput(stateSpace.velocity)
                }
            }
        }
        part pusher : Pusher {
            attribute pusherForce :> ISQ::force;

            action pusherBehavior : ContinuousStateSpaceDynamics {
                in input;
                out output : PusherOutput;
                calc :>> getOutput {
                    new PusherOutput(pusherForce)
                }
            }
        }

        flow pusher.pusherBehavior.output to cart.cartBehavior.input;
    }
}
~~~
# TOKENS
~~~zig
LineComment,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,Ident,OpenParen,Ident,Dot,Ident,Slash,Ident,CloseParen,
CloseCurly,
KwCalc,ColonGtGt,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,Colon,Ident,Semicolon,
Ident,Ident,OpenParen,Ident,Dot,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,OpenCurly,
Ident,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (line_comment)
  (package_def 'CartSample'
    (import_decl private 'StateSpaceRepresentation::*')
    (part_def 'Cart'
      (attribute_usage 'mass' :> 'ISQ::mass')
      (attribute_def 'CartInput' :> 'Input'
        (attribute_usage 'force' :> 'ISQ::force'))
      (attribute_def 'CartOutput' :> 'Output'
        (attribute_usage 'velocity' :> 'ISQ::speed'))
      (attribute_def 'CartState' :> 'StateSpace'
        (attribute_usage 'velocity' :> 'ISQ::speed'))
      (attribute_def 'CartStateDerivative' :> 'StateDerivative'
        (ref_usage ref :>> 'stateSpace' : 'CartState')
        (attribute_usage 'accel' :> 'ISQ::acceleration')))
    (part_def 'Pusher'
      (attribute_def 'PusherOutput' :> 'Output'
        (attribute_usage 'force' :> 'ISQ::force')))
    (part_usage 'context'
      (part_usage 'cart' : 'Cart'
        (action_usage 'cartBehavior' : 'ContinuousStateSpaceDynamics'
          (default_ref_usage in 'input' : 'CartInput')
          (default_ref_usage out 'output' : 'CartOutput')
          (default_ref_usage :>> 'stateSpace' : 'CartState')
          (calc_usage :>> 'getDerivative'
            (default_ref_usage in 'input' : 'CartInput')
            (default_ref_usage in 'stateSpace' : 'CartState')
            (result_expr_member))
          (calc_usage :>> 'getOutput'
            (default_ref_usage in :>> 'stateSpace' : 'CartState')
            (result_expr_member))))
      (part_usage 'pusher' : 'Pusher'
        (attribute_usage 'pusherForce' :> 'ISQ::force')
        (action_usage 'pusherBehavior' : 'ContinuousStateSpaceDynamics'
          (default_ref_usage in 'input')
          (default_ref_usage out 'output' : 'PusherOutput')
          (calc_usage :>> 'getOutput'
            (result_expr_member))))
      (flow_usage 'pusher'))))
~~~
# FORMAT
~~~sysml
// State Space Representation cart example

package CartSample {
    private import StateSpaceRepresentation::*;
    part def Cart {
        attribute mass :> ISQ::mass;

        attribute def CartInput :> Input {
            attribute force :> ISQ::force;
        }

        attribute def CartOutput :> Output {
            attribute velocity :> ISQ::speed;
        }

        attribute def CartState :> StateSpace {
            attribute velocity :> ISQ::speed;
        }

        attribute def CartStateDerivative :> StateDerivative {
            ref :>> stateSpace : CartState;
            attribute accel :> ISQ::acceleration;
        }
    }

    part def Pusher {
        attribute def PusherOutput :> Output {
            attribute force :> ISQ::force;
        }
    }

    part context {
        part cart : Cart {
            action cartBehavior : ContinuousStateSpaceDynamics {
                in input : CartInput;
                out output : CartOutput;
                :>> stateSpace : CartState;

                calc :>> getDerivative {
                    in input: CartInput;
                    in stateSpace: CartState;
                    new CartStateDerivative(input.force / mass)
                }
                calc :>> getOutput {
                    in :>> stateSpace : CartState;
                    new CartOutput(stateSpace.velocity)
                }
            }
        }
        part pusher : Pusher {
            attribute pusherForce :> ISQ::force;

            action pusherBehavior : ContinuousStateSpaceDynamics {
                in input;
                out output : PusherOutput;
                calc :>> getOutput {
                    new PusherOutput(pusherForce)
                }
            }
        }

        flow pusher.pusherBehavior.output to cart.cartBehavior.input;
    }
}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'pusher'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'StateDerivative'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'getDerivative'
semantic.unresolved_name 'getOutput'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'getOutput'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'pusher'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'StateDerivative'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'getDerivative'
semantic.unresolved_name 'getOutput'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'getOutput'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CartSample"))) (name "CartSample") (declared-name "CartSample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CartSample::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CartSample::Cart"))) (name "Cart") (declared-name "Cart") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))) (name "CartInput") (declared-name "CartInput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))) (name "CartOutput") (declared-name "CartOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "CartSample::Cart::CartState"))) (name "CartState") (declared-name "CartState") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))) (name "CartStateDerivative") (declared-name "CartStateDerivative") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CartSample::Cart::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "CartSample::Pusher"))) (name "Pusher") (declared-name "Pusher") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))) (name "PusherOutput") (declared-name "PusherOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "CartSample::context"))) (name "context") (declared-name "context") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "CartSample::context::cart"))) (name "cart") (declared-name "cart") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (name "cartBehavior") (declared-name "cartBehavior") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart"))))
                  (contains
                    (element (kind "action body decl") (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getDerivative"))) (name ":>> getDerivative") (declared-name ":>> getDerivative") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
                    (element (kind "action body decl") (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getOutput"))) (name ":>> getOutput") (declared-name ":>> getOutput") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Cart")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "CartSample::context::pusher"))) (name "pusher") (declared-name "pusher") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (name "pusherBehavior") (declared-name "pusherBehavior") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher"))))
                  (contains
                    (element (kind "action body decl") (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior:::>> getOutput"))) (name ":>> getOutput") (declared-name ":>> getOutput") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))) (name "pusherForce") (declared-name "pusherForce") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CartSample::Pusher")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (to (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (flow (source-expression "pusher::pusherBehavior::output") (target-expression "cart::cartBehavior::input")))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::cart"))) (to (node (document "d0") (qualified-name "CartSample::Cart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (to (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (to (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::pusher"))) (to (node (document "d0") (qualified-name "CartSample::Pusher"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (to (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/cart_sample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 4) (end 3 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 35) (end 7 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 36) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 35) (end 15 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 45) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 38) (end 26 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 12) (end 33 567))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 12) (end 52 254))
      )
    )
  )
)
~~~
