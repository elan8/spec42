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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "cart_sample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 26) (end 5 35))
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
        (range (start 34 16) (end 34 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 16) (end 35 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 37) (end 50 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 12) (end 52 254))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 16) (end 53 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 16) (end 54 42))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eda2e50772f90fd42a42bc20a4d7483b0a5851ad6afe4dcda1cc34b2bbc06e36") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CartSample"))) (kind "package") (name "CartSample") (declared-name "CartSample") (range (start (line 2) (character 0)) (end (line 2) (character 1782))))
    (element (id (node (document "d0") (qualified-name "CartSample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 47))) (parent (node (document "d0") (qualified-name "CartSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "StateSpaceRepresentation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 43))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart"))) (kind "part def") (name "Cart") (declared-name "Cart") (range (start (line 4) (character 4)) (end (line 4) (character 534))) (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))) (kind "attribute def") (name "CartInput") (declared-name "CartInput") (range (start (line 7) (character 8)) (end (line 7) (character 95))) (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input") (range (start (line 7) (character 35)) (end (line 7) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))) (kind "attribute def") (name "CartOutput") (declared-name "CartOutput") (range (start (line 11) (character 8)) (end (line 11) (character 100))) (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 11) (character 36)) (end (line 11) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartState"))) (kind "attribute def") (name "CartState") (declared-name "CartState") (range (start (line 15) (character 8)) (end (line 15) (character 103))) (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace") (range (start (line 15) (character 35)) (end (line 15) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind "attribute def") (name "CartStateDerivative") (declared-name "CartStateDerivative") (range (start (line 19) (character 8)) (end (line 19) (character 166))) (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateDerivative") (range (start (line 19) (character 45)) (end (line 19) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 5) (character 8)) (end (line 5) (character 36))) (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 5) (character 26)) (end (line 5) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::Pusher"))) (kind "part def") (name "Pusher") (declared-name "Pusher") (range (start (line 25) (character 4)) (end (line 25) (character 127))) (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind "attribute def") (name "PusherOutput") (declared-name "PusherOutput") (range (start (line 26) (character 8)) (end (line 26) (character 99))) (parent (node (document "d0") (qualified-name "CartSample::Pusher"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 26) (character 38)) (end (line 26) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::context"))) (kind "part") (name "context") (declared-name "context") (range (start (line 31) (character 4)) (end (line 31) (character 1046))) (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind "part") (name "cart") (declared-name "cart") (range (start (line 32) (character 8)) (end (line 32) (character 604))) (parent (node (document "d0") (qualified-name "CartSample::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cart") (range (start (line 32) (character 20)) (end (line 32) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind "action") (name "cartBehavior") (declared-name "cartBehavior") (range (start (line 33) (character 12)) (end (line 33) (character 567))) (parent (node (document "d0") (qualified-name "CartSample::context::cart"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getDerivative"))) (kind "action body decl") (name ":>> getDerivative") (declared-name ":>> getDerivative") (range (start (line 38) (character 16)) (end (line 38) (character 205))) (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getOutput"))) (kind "action body decl") (name ":>> getOutput") (declared-name ":>> getOutput") (range (start (line 43) (character 16)) (end (line 43) (character 158))) (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 34) (character 16)) (end (line 34) (character 37))) (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (authored (relationships (typing (reference "CartInput") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 35) (character 16)) (end (line 35) (character 40))) (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (authored (relationships (typing (reference "CartOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind "part") (name "pusher") (declared-name "pusher") (range (start (line 49) (character 8)) (end (line 49) (character 345))) (parent (node (document "d0") (qualified-name "CartSample::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pusher") (range (start (line 49) (character 22)) (end (line 49) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind "action") (name "pusherBehavior") (declared-name "pusherBehavior") (range (start (line 52) (character 12)) (end (line 52) (character 254))) (parent (node (document "d0") (qualified-name "CartSample::context::pusher"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior:::>> getOutput"))) (kind "action body decl") (name ":>> getOutput") (declared-name ":>> getOutput") (range (start (line 55) (character 16)) (end (line 55) (character 104))) (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 53) (character 16)) (end (line 53) (character 25))) (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 54) (character 16)) (end (line 54) (character 42))) (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (authored (relationships (typing (reference "PusherOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind "attribute") (name "pusherForce") (declared-name "pusherForce") (range (start (line 50) (character 12)) (end (line 50) (character 48))) (parent (node (document "d0") (qualified-name "CartSample::context::pusher"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::force") (range (start (line 50) (character 37)) (end (line 50) (character 47)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CartSample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "StateSpaceRepresentation::*") (range (start (line 3) (character 19)) (end (line 3) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range (start (line 7) (character 35)) (end (line 7) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 11) (character 36)) (end (line 11) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range (start (line 15) (character 35)) (end (line 15) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "StateDerivative") (range (start (line 19) (character 45)) (end (line 19) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 5) (character 26)) (end (line 5) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 26) (character 38)) (end (line 26) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowSource) (ordinal 0)) (authored-target "pusher::pusherBehavior::output") (range (start (line 61) (character 13)) (end (line 61) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowTarget) (ordinal 0)) (authored-target "cart::cartBehavior::input") (range (start (line 61) (character 45)) (end (line 61) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "Cart") (range (start (line 32) (character 20)) (end (line 32) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::Cart")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "CartInput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "CartOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0)) (authored-target "Pusher") (range (start (line 49) (character 22)) (end (line 49) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::Pusher")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "PusherOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::force") (range (start (line 50) (character 37)) (end (line 50) (character 47))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (target (node (document "d0") (qualified-name "CartSample::Cart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (target (node (document "d0") (qualified-name "CartSample::Pusher"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (target (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "pusher::pusherBehavior::output") (target "cart::cartBehavior::input") (source-range (start (line 61) (character 13)) (end (line 61) (character 41))) (target-range (start (line 61) (character 45)) (end (line 61) (character 68)))))
  )
  (evaluation
  )
)
~~~
