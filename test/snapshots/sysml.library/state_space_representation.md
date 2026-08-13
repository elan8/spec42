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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 4) (end 20 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 4) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 49 8) (end 49 34))
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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 55 8) (end 55 68))
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
        (range (start 64 8) (end 64 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 65 8) (end 65 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 4) (end 77 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 79 4) (end 91 2))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 117 4) (end 127 2))
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f33163e1dfdd1649bfdb24539884e26d75c8e3572b32d3309297c3a46604caab") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::DurationValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VectorCalculations") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
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
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Output"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind specialization) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpaceEventDef")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/state_space_representation.md") (range (start 12 36) (end 12 55)) (probe (position 12 36))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::Input"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
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
  (query (document "memory://snapshot/state_space_representation.md") (range (start 11 41) (end 11 60)) (probe (position 11 41))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind specialization) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/state_space_representation.md") (range (start 33 39) (end 33 57)) (probe (position 33 39))
    (reference (id (source (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0) (authored-target "StateSpaceEventDef")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_space_representation.md") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
  )
)
~~~
