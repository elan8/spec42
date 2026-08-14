# META
~~~ini
description=Standard Library: Domain Libraries/Analysis/StateSpaceRepresentation
type=file
~~~
# SOURCE
~~~sysml
standard library package StateSpaceRepresentation {
	doc
	/*
	 * This package provides a model of the foundational state-space system representation, 
	 * commonly used in control systems.
	 */

    private import ISQ::DurationValue;
    private import Quantities::VectorQuantityValue;
    private import VectorCalculations::*;

    abstract attribute def StateSpace :> VectorQuantityValue;
    abstract attribute def Input :> VectorQuantityValue;
    abstract attribute def Output :> VectorQuantityValue;

    abstract calc def GetNextState { 
    	in input: Input; 
    	in stateSpace: StateSpace; 
    	in timeStep: DurationValue; 
    	return : StateSpace;
    }
    abstract calc def GetOutput {
    	in input: Input; 
    	in stateSpace: StateSpace;
    	return : Output;
	}
	
     abstract action def StateSpaceEventDef {
    	doc
    	/*
    	 * Events to be received.
    	 */
    }
    action def ZeroCrossingEventDef :> StateSpaceEventDef;

    item def StateSpaceItem {
    	doc
    	/*
    	 * Item for SSR connection
    	 */
    }

    abstract action def StateSpaceDynamics {
	    doc
	    /*
	     * StateSpaceDynamics is the simplest form of State Space Representation,
	     * and nextState directly computes the stateSpace of the next timestep. 
	     */
    
        in attribute input: Input;

        abstract calc getNextState: GetNextState;
        abstract calc getOutput: GetOutput;
        attribute stateSpace: StateSpace;

        out attribute output: Output = getOutput(input, stateSpace);
    }

    abstract attribute def StateDerivative :> VectorQuantityValue {
	    doc
	    /*
	     * The definition of the time derivative of StateSpace, which means dx/dt, where x is StateSpace
	     */
    
        ref stateSpace: StateSpace;
        constraint { stateSpace.order == order }
    }

    abstract calc def GetDerivative {
	    doc
	    /*
	     * Computes the time derivative of stateSpace, which corresponds dx/dt = f(u, x), where u is input and x is stateSpace.
	     */
	    
    	in input: Input;
    	in stateSpace: StateSpace;
    	return : StateDerivative;
	}
	
    abstract calc def Integrate {
	    doc
	    /*
	     * Integrates stateSpace to compute the next stateSpace, which corresponds to x + int dx/dt dt.
	     * Its actual implementation should be given by a solver. 
	     */
    
        in getDerivative: GetDerivative;
        in input: Input;
        in initialState: StateSpace;
        in timeInterval: DurationValue;
        return result: StateSpace;
	}
	
    abstract action def ContinuousStateSpaceDynamics :> StateSpaceDynamics {
	    doc
	    /*
	     * ContinuousStateSpaceDynamics represents continuous behavior.
	     * derivative needs to return a time derivative of stateSpace, i.e. dx/dt.
	     */
    
        abstract calc getDerivative: GetDerivative;
        calc :>> getNextState: GetNextState {
            /* We compute nextState by Integrate defined above by giving derivative calc. */
            calc integrate: Integrate {
                in getDerivative = ContinuousStateSpaceDynamics::getDerivative;
                in input = GetNextState::input;
                in initialState = GetNextState::stateSpace;
                in timeInterval = GetNextState::timeStep;
           }
           return result = integrate.result;
        }

        event occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef {
        	/* ContinuousStateSpaceDynamics may cause zero crossings anomaly. */ 
        }
    }

    abstract calc def GetDifference {
    	doc
	    /*
	     * Computes difference of stateSpace by one timestep, that is x(k+1) - x(k),
	     * where k is the timestep number. 
	     */
    
    	in input: Input;
    	in stateSpace: StateSpace;
    	return : StateSpace;
	}
	
    abstract action def DiscreteStateSpaceDynamics :> StateSpaceDynamics {
	    doc
	    /*
	     * DiscreteStateSpaceDynamics represents discrete behavior.
	     * differences needs to return difference of the stateSpace for each timestep.
	     */
    
        abstract calc getDifference: GetDifference;
        calc :>> getNextState: GetNextState {
            attribute diff: StateSpace = getDifference(input, stateSpace);
            return result = stateSpace + diff;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_space_representation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 41) (end 11 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 36) (end 12 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 37) (end 13 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 18) (end 18 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 51 8) (end 51 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 52 8) (end 52 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 53 8) (end 53 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 39) (end 55 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 56) (end 55 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 58 46) (end 58 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 65 8) (end 65 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 25) (end 89 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 100 8) (end 100 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 101 8) (end 110 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 112 8) (end 114 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 136 8) (end 136 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 137 8) (end 140 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:f33163e1dfdd1649bfdb24539884e26d75c8e3572b32d3309297c3a46604caab") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::DurationValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VectorCalculations") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateDerivative"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Output"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GetDerivative") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Input") (direction in))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Output") (direction out)) (expressionOperand (reference "input")) (expressionOperand (reference "stateSpace")) (invocationCallee (reference "getOutput"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpaceEventDef"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VectorCalculations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpaceDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpaceDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "StateDerivative")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Output")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0))
      (authored-target "GetDerivative")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "Output")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind expressionOperand) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input")))))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind expressionOperand) (ordinal 1))
      (authored-target "stateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind invocationCallee) (ordinal 0))
      (authored-target "getOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpaceEventDef")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_space_representation.md") (range (start 9 19) (end 9 40)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "VectorCalculations")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 7 19) (end 7 37)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 8 19) (end 8 50)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 93 56) (end 93 74)) (probe (position 93 56))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0) (authored-target "StateSpaceDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 129 54) (end 129 72)) (probe (position 129 54))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0) (authored-target "StateSpaceDynamics")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 76 14) (end 76 29)) (probe (position 76 14))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "StateDerivative")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 74 15) (end 74 20)) (probe (position 74 15))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 75 20) (end 75 30)) (probe (position 75 20))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 126 14) (end 126 24)) (probe (position 126 14))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 124 15) (end 124 20)) (probe (position 124 15))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 125 20) (end 125 30)) (probe (position 125 20))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 19 14) (end 19 24)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 16 15) (end 16 20)) (probe (position 16 15))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 17 20) (end 17 30)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 18 18) (end 18 31)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 24 14) (end 24 20)) (probe (position 24 14))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Output")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 22 15) (end 22 20)) (probe (position 22 15))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 23 20) (end 23 30)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 12 36) (end 12 55)) (probe (position 12 36))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 86 26) (end 86 39)) (probe (position 86 26))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0) (authored-target "GetDerivative")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 88 25) (end 88 35)) (probe (position 88 25))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 87 18) (end 87 23)) (probe (position 87 18))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 90 23) (end 90 33)) (probe (position 90 23))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 89 25) (end 89 38)) (probe (position 89 25))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 13 37) (end 13 56)) (probe (position 13 37))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 58 46) (end 58 65)) (probe (position 58 46))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 64 24) (end 64 34)) (probe (position 64 24))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 11 41) (end 11 60)) (probe (position 11 41))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 49 28) (end 49 33)) (probe (position 49 28))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0) (authored-target "Input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 55 30) (end 55 36)) (probe (position 55 30))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0) (authored-target "Output")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 55 49) (end 55 54)) (probe (position 55 49))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind expressionOperand) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input")))))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 55 56) (end 55 66)) (probe (position 55 56))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind expressionOperand) (ordinal 1) (authored-target "stateSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 55 39) (end 55 48)) (probe (position 55 39))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind invocationCallee) (ordinal 0) (authored-target "getOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 33 39) (end 33 57)) (probe (position 33 39))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0) (authored-target "StateSpaceEventDef")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
  )
)
~~~
