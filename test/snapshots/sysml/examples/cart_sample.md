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
    (element (id (node (document "d0") (qualified-name "CartSample"))) (kind "package") (name "CartSample") (declared-name "CartSample"))
    (element (id (node (document "d0") (qualified-name "CartSample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "CartSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "StateSpaceRepresentation::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart"))) (kind "part def") (name "Cart") (declared-name "Cart") (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))) (kind "attribute def") (name "CartInput") (declared-name "CartInput") (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))) (kind "attribute def") (name "CartOutput") (declared-name "CartOutput") (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartState"))) (kind "attribute def") (name "CartState") (declared-name "CartState") (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind "attribute def") (name "CartStateDerivative") (declared-name "CartStateDerivative") (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateDerivative")))))
    (element (id (node (document "d0") (qualified-name "CartSample::Cart::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "CartSample::Cart"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "CartSample::Pusher"))) (kind "part def") (name "Pusher") (declared-name "Pusher") (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind "attribute def") (name "PusherOutput") (declared-name "PusherOutput") (parent (node (document "d0") (qualified-name "CartSample::Pusher"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context"))) (kind "part") (name "context") (declared-name "context") (parent (node (document "d0") (qualified-name "CartSample"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind "part") (name "cart") (declared-name "cart") (parent (node (document "d0") (qualified-name "CartSample::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cart")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind "action") (name "cartBehavior") (declared-name "cartBehavior") (parent (node (document "d0") (qualified-name "CartSample::context::cart"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getDerivative"))) (kind "action body decl") (name ":>> getDerivative") (declared-name ":>> getDerivative") (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior:::>> getOutput"))) (kind "action body decl") (name ":>> getOutput") (declared-name ":>> getOutput") (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (authored (relationships (typing (reference "CartInput")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (authored (relationships (typing (reference "CartOutput")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind "part") (name "pusher") (declared-name "pusher") (parent (node (document "d0") (qualified-name "CartSample::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pusher")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind "action") (name "pusherBehavior") (declared-name "pusherBehavior") (parent (node (document "d0") (qualified-name "CartSample::context::pusher"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior:::>> getOutput"))) (kind "action body decl") (name ":>> getOutput") (declared-name ":>> getOutput") (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (authored (relationships (typing (reference "PusherOutput")))))
    (element (id (node (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind "attribute") (name "pusherForce") (declared-name "pusherForce") (parent (node (document "d0") (qualified-name "CartSample::context::pusher"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::force")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CartSample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "StateSpaceRepresentation::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "StateDerivative") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Cart::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowSource) (ordinal 0)) (authored-target "pusher::pusherBehavior::output") (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowTarget) (ordinal 0)) (authored-target "cart::cartBehavior::input") (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "Cart") (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::Cart")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "CartInput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "CartOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0)) (authored-target "Pusher") (outcome (status resolved) (target (node (document "d0") (qualified-name "CartSample::Pusher")))))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "PusherOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::force") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (target (node (document "d0") (qualified-name "CartSample::Cart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (target (node (document "d0") (qualified-name "CartSample::Pusher"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (target (node (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CartSample::context"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "pusher::pusherBehavior::output") (target "cart::cartBehavior::input")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 32 20) (end 32 24)) (probe (position 32 20))
      (reference
        (source (document "d0") (qualified-name "CartSample::context::cart"))
        (kind featureTyping) (ordinal 0) (authored-target "Cart")
        (range (start 32 20) (end 32 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CartSample::Cart") (range (start 4 4) (end 4 534)))
        )
      )
    )
    (query (range (start 7 35) (end 7 40)) (probe (position 7 35))
      (reference
        (source (document "d0") (qualified-name "CartSample::Cart::CartInput"))
        (kind featureTyping) (ordinal 0) (authored-target "Input")
        (range (start 7 35) (end 7 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 36) (end 11 42)) (probe (position 11 36))
      (reference
        (source (document "d0") (qualified-name "CartSample::Cart::CartOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 11 36) (end 11 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 38) (end 26 44)) (probe (position 26 38))
      (reference
        (source (document "d0") (qualified-name "CartSample::Pusher::PusherOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 26 38) (end 26 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 49 22) (end 49 28)) (probe (position 49 22))
      (reference
        (source (document "d0") (qualified-name "CartSample::context::pusher"))
        (kind featureTyping) (ordinal 0) (authored-target "Pusher")
        (range (start 49 22) (end 49 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CartSample::Pusher") (range (start 25 4) (end 25 127)))
        )
      )
    )
    (query (range (start 5 26) (end 5 35)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "CartSample::Cart::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 5 26) (end 5 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 35) (end 15 45)) (probe (position 15 35))
      (reference
        (source (document "d0") (qualified-name "CartSample::Cart::CartState"))
        (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
        (range (start 15 35) (end 15 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 50 37) (end 50 47)) (probe (position 50 37))
      (reference
        (source (document "d0") (qualified-name "CartSample::context::pusher::pusherForce"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
        (range (start 50 37) (end 50 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 45) (end 19 60)) (probe (position 19 45))
      (reference
        (source (document "d0") (qualified-name "CartSample::Cart::CartStateDerivative"))
        (kind featureTyping) (ordinal 0) (authored-target "StateDerivative")
        (range (start 19 45) (end 19 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 45) (end 61 68)) (probe (position 61 45))
      (reference
        (source (document "d0") (qualified-name "CartSample::context"))
        (kind flowTarget) (ordinal 0) (authored-target "cart::cartBehavior::input")
        (range (start 61 45) (end 61 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CartSample::context::cart::cartBehavior::input") (range (start 34 16) (end 34 37)))
        )
      )
    )
    (query (range (start 3 19) (end 3 43)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "CartSample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "StateSpaceRepresentation::*")
        (range (start 3 19) (end 3 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 13) (end 61 41)) (probe (position 61 13))
      (reference
        (source (document "d0") (qualified-name "CartSample::context"))
        (kind flowSource) (ordinal 0) (authored-target "pusher::pusherBehavior::output")
        (range (start 61 13) (end 61 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CartSample::context::pusher::pusherBehavior::output") (range (start 54 16) (end 54 42)))
        )
      )
    )
  )
)
~~~
