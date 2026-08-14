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
  (document "memory://snapshot/cart_sample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 26) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 35) (end 7 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 31) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 36) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 34) (end 12 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 35) (end 15 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 34) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 45) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 20) (end 20 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 31) (end 21 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 38) (end 26 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 31) (end 27 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 34) (end 33 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 27) (end 34 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 29) (end 35 39))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 36 16) (end 38 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 38 16) (end 42 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 43 16) (end 46 17))
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
        (range (start 52 36) (end 52 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 29) (end 54 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 55 16) (end 57 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 61 8) (end 61 69))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c3bdbea9954bdb3884e6d5550ccc8cd362300b6f3b9709cf4bf84f6e0c3603e9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StateSpaceRepresentation") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Input")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput::force"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::force")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpace")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateDerivative")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartState")) (redefinition (reference "stateSpace")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative::accel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::acceleration")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput::force"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::force")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cart")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartInput") (direction in)))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartOutput") (direction out)))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pusher")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics")))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PusherOutput") (direction out)))))
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::force")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StateSpaceRepresentation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput"))) (kind specialization) (ordinal 0))
      (authored-target "Input")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput::force"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput::velocity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState::velocity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind specialization) (ordinal 0))
      (authored-target "StateDerivative")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartState")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState")))))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "stateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative::accel"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput::force"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart")))))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartInput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pusher")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher")))))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "PusherOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::force")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart"))) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher"))) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart")))
      (supertype (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher")))
      (supertype (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cart_sample.md") (range (start 3 19) (end 3 46)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StateSpaceRepresentation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 7 35) (end 7 40)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput"))) (kind specialization) (ordinal 0) (authored-target "Input")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 8 31) (end 8 41)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartInput::force"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 11 36) (end 11 42)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 12 34) (end 12 44)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartOutput::velocity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 15 35) (end 15 45)) (probe (position 15 35))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState"))) (kind specialization) (ordinal 0) (authored-target "StateSpace")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 16 34) (end 16 44)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState::velocity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 19 45) (end 19 60)) (probe (position 19 45))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative"))) (kind specialization) (ordinal 0) (authored-target "StateDerivative")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 20 33) (end 20 42)) (probe (position 20 33))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "CartState")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartState")))))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 20 20) (end 20 30)) (probe (position 20 20))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (path (named (kind package) (name "CartSample")) (named (kind part-def) (name "Cart")) (named (kind attribute-def) (name "CartStateDerivative")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "stateSpace")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 21 31) (end 21 48)) (probe (position 21 31))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::CartStateDerivative::accel"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 5 26) (end 5 35)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 26 38) (end 26 44)) (probe (position 26 38))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 27 31) (end 27 41)) (probe (position 27 31))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher::PusherOutput::force"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 32 20) (end 32 24)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart"))) (kind featureTyping) (ordinal 0) (authored-target "Cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Cart")))))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 33 34) (end 33 62)) (probe (position 33 34))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 34 27) (end 34 36)) (probe (position 34 27))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::input"))) (kind featureTyping) (ordinal 0) (authored-target "CartInput")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 35 29) (end 35 39)) (probe (position 35 29))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::cart::cartBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "CartOutput")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 49 22) (end 49 28)) (probe (position 49 22))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher"))) (kind featureTyping) (ordinal 0) (authored-target "Pusher")
      (outcome (status resolved) (target (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::Pusher")))))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 52 36) (end 52 64)) (probe (position 52 36))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 54 29) (end 54 41)) (probe (position 54 29))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "PusherOutput")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cart_sample.md") (range (start 50 37) (end 50 47)) (probe (position 50 37))
    (reference (id (source (node (document "memory://snapshot/cart_sample.md") (qualified-name "CartSample::context::pusher::pusherForce"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
      (outcome (status unresolved)))
    )
  )
)
~~~
