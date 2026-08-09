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
                    in input : CartInput;
                    in stateSpace : CartState;
                    = new CartStateDerivative(input.force / mass);
                }
                calc :>> getOutput {
                    in :>> stateSpace : CartState;
                    = new CartOutput(stateSpace.velocity);
                }
            }
        }
        part pusher : Pusher {
            attribute pusherForce :> ISQ::force;

            action pusherBehavior : ContinuousStateSpaceDynamics {
                in input;
                out output : PusherOutput;
                calc :>> getOutput {
                    = new PusherOutput(pusherForce);
                }
            }
        }

        flow pusher;
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
(model
  (namespace
    (package 'CartSample'
      (namespace_import private -> 'StateSpaceRepresentation'[unresolved])
      (part_def 'Cart'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
        (attribute_def 'CartInput' :> 'Input'[unresolved]
          (attribute_usage composite 'force' :> 'ISQ::force'[unresolved]))
        (attribute_def 'CartOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'velocity' :> 'ISQ::speed'[unresolved]))
        (attribute_def 'CartState' :> 'StateSpace'[unresolved]
          (attribute_usage composite 'velocity' :> 'ISQ::speed'[unresolved]))
        (attribute_def 'CartStateDerivative' :> 'StateDerivative'[unresolved]
          (reference_usage reference :>> 'stateSpace'[unresolved] : 'CartSample::Cart::CartState'[attribute_def])
          (attribute_usage composite 'accel' :> 'ISQ::acceleration'[unresolved])))
      (part_def 'Pusher'
        (attribute_def 'PusherOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'force' :> 'ISQ::force'[unresolved])))
      (part_usage 'context'
        (part_usage composite 'cart' : 'CartSample::Cart'[part_def]
          (action_usage composite 'cartBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
            (reference_usage in reference 'input' : 'CartSample::Cart::CartInput'[attribute_def])
            (reference_usage out reference 'output' : 'CartSample::Cart::CartOutput'[attribute_def])
            (reference_usage reference :>> 'stateSpace'[unresolved] : 'CartSample::Cart::CartState'[attribute_def])
            (calculation_usage composite :>> 'getDerivative'[unresolved]
              (reference_usage in reference 'input' : 'CartSample::Cart::CartInput'[attribute_def])
              (reference_usage in reference 'stateSpace' : 'CartSample::Cart::CartState'[attribute_def])
              (result_expr_membership))
            (calculation_usage composite :>> 'getOutput'[unresolved]
              (reference_usage in reference :>> 'stateSpace'[unresolved] : 'CartSample::Cart::CartState'[attribute_def])
              (result_expr_membership))))
        (part_usage composite 'pusher' : 'CartSample::Pusher'[part_def]
          (attribute_usage composite 'pusherForce' :> 'ISQ::force'[unresolved])
          (action_usage composite 'pusherBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
            (reference_usage in reference 'input')
            (reference_usage out reference 'output' : 'CartSample::Pusher::PusherOutput'[attribute_def])
            (calculation_usage composite :>> 'getOutput'[unresolved]
              (result_expr_membership))))
        (flow_usage composite 'pusher')))))
~~~
