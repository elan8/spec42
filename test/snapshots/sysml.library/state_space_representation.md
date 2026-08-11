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
  (document "state_space_representation.md"
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
        (range (start 9 19) (end 9 37))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5be6f48545c9ebacd2c5b8ef64368d8c0d2b58d510858162aeb531b466f8e7dc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (kind "package") (name "StateSpaceRepresentation") (declared-name "StateSpaceRepresentation") (range (start (line 0) (character 0)) (end (line 0) (character 4263))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 4)) (end (line 9) (character 41))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorCalculations::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 37))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind "action def") (name "ContinuousStateSpaceDynamics") (declared-name "ContinuousStateSpaceDynamics") (range (start (line 93) (character 4)) (end (line 93) (character 972))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceDynamics") (range none)) (specializes (reference "StateSpaceDynamics") (range none)) (specializes (reference "StateSpaceDynamics") (range (start (line 93) (character 56)) (end (line 93) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics:::>> getNextState: GetNextState"))) (kind "action body decl") (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (range (start (line 101) (character 8)) (end (line 101) (character 492))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (range (start (line 93) (character 4)) (end (line 93) (character 972))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::getDerivative: GetDerivative"))) (kind "action body decl") (name "getDerivative: GetDerivative") (declared-name "getDerivative: GetDerivative") (range (start (line 100) (character 8)) (end (line 100) (character 51))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef"))) (kind "action body decl") (name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (declared-name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (range (start (line 112) (character 8)) (end (line 112) (character 163))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind "action def") (name "DiscreteStateSpaceDynamics") (declared-name "DiscreteStateSpaceDynamics") (range (start (line 129) (character 4)) (end (line 129) (character 490))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceDynamics") (range none)) (specializes (reference "StateSpaceDynamics") (range none)) (specializes (reference "StateSpaceDynamics") (range (start (line 129) (character 54)) (end (line 129) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics:::>> getNextState: GetNextState"))) (kind "action body decl") (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (range (start (line 137) (character 8)) (end (line 137) (character 177))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (range (start (line 129) (character 4)) (end (line 129) (character 490))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::getDifference: GetDifference"))) (kind "action body decl") (name "getDifference: GetDifference") (declared-name "getDifference: GetDifference") (range (start (line 136) (character 8)) (end (line 136) (character 51))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (range (start (line 7) (character 4)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 37))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (kind "calc def") (name "GetDerivative") (declared-name "GetDerivative") (range (start (line 68) (character 4)) (end (line 68) (character 282))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (kind "return parameter") (name "") (range (start (line 76) (character 5)) (end (line 76) (character 30))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "StateDerivative") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::_documentation"))) (kind "documentation") (name "") (range (start (line 68) (character 4)) (end (line 68) (character 282))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 74) (character 5)) (end (line 74) (character 21))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (range (start (line 75) (character 5)) (end (line 75) (character 31))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (kind "calc def") (name "GetDifference") (declared-name "GetDifference") (range (start (line 117) (character 4)) (end (line 117) (character 274))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (kind "return parameter") (name "") (range (start (line 126) (character 5)) (end (line 126) (character 25))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::_documentation"))) (kind "documentation") (name "") (range (start (line 117) (character 4)) (end (line 117) (character 274))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 124) (character 5)) (end (line 124) (character 21))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (range (start (line 125) (character 5)) (end (line 125) (character 31))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (kind "calc def") (name "GetNextState") (declared-name "GetNextState") (range (start (line 15) (character 4)) (end (line 15) (character 159))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (kind "return parameter") (name "") (range (start (line 19) (character 5)) (end (line 19) (character 25))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 16) (character 5)) (end (line 16) (character 21))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (range (start (line 17) (character 5)) (end (line 17) (character 31))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind "in out parameter") (name "timeStep") (declared-name "timeStep") (range (start (line 18) (character 5)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (kind "calc def") (name "GetOutput") (declared-name "GetOutput") (range (start (line 21) (character 4)) (end (line 21) (character 113))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (kind "return parameter") (name "") (range (start (line 24) (character 5)) (end (line 24) (character 21))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "Output") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 22) (character 5)) (end (line 22) (character 21))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (range (start (line 23) (character 5)) (end (line 23) (character 31))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (kind "attribute def") (name "Input") (declared-name "Input") (range (start (line 12) (character 4)) (end (line 12) (character 56))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (kind "calc def") (name "Integrate") (declared-name "Integrate") (range (start (line 79) (character 4)) (end (line 79) (character 410))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 4)) (end (line 79) (character 410))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind "in out parameter") (name "getDerivative") (declared-name "getDerivative") (range (start (line 86) (character 8)) (end (line 86) (character 40))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "GetDerivative") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind "in out parameter") (name "initialState") (declared-name "initialState") (range (start (line 88) (character 8)) (end (line 88) (character 36))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 87) (character 8)) (end (line 87) (character 24))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind "return parameter") (name "result") (declared-name "result") (range (start (line 90) (character 8)) (end (line 90) (character 34))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "StateSpace") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind "in out parameter") (name "timeInterval") (declared-name "timeInterval") (range (start (line 89) (character 8)) (end (line 89) (character 39))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (kind "attribute def") (name "Output") (declared-name "Output") (range (start (line 13) (character 4)) (end (line 13) (character 57))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind "attribute def") (name "StateDerivative") (declared-name "StateDerivative") (range (start (line 58) (character 4)) (end (line 58) (character 291))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::_documentation"))) (kind "documentation") (name "") (range (start (line 58) (character 4)) (end (line 58) (character 291))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind "ref") (name "stateSpace") (declared-name "stateSpace") (range (start (line 64) (character 8)) (end (line 64) (character 35))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateSpace") (range (start (line 64) (character 23)) (end (line 64) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind "attribute def") (name "StateSpace") (declared-name "StateSpace") (range (start (line 11) (character 4)) (end (line 11) (character 61))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (kind "action def") (name "StateSpaceDynamics") (declared-name "StateSpaceDynamics") (range (start (line 42) (character 4)) (end (line 42) (character 480))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (range (start (line 42) (character 4)) (end (line 42) (character 480))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getNextState: GetNextState"))) (kind "action body decl") (name "getNextState: GetNextState") (declared-name "getNextState: GetNextState") (range (start (line 51) (character 8)) (end (line 51) (character 49))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getOutput: GetOutput"))) (kind "action body decl") (name "getOutput: GetOutput") (declared-name "getOutput: GetOutput") (range (start (line 52) (character 8)) (end (line 52) (character 43))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 49) (character 8)) (end (line 49) (character 34))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (authored (relationships (typing (reference "Input") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 55) (character 8)) (end (line 55) (character 68))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (authored (relationships (typing (reference "Output") (range none)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::stateSpace: StateSpace"))) (kind "action body decl") (name "stateSpace: StateSpace") (declared-name "stateSpace: StateSpace") (range (start (line 53) (character 8)) (end (line 53) (character 41))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (kind "action def") (name "StateSpaceEventDef") (declared-name "StateSpaceEventDef") (range (start (line 27) (character 5)) (end (line 27) (character 108))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef::_documentation"))) (kind "documentation") (name "") (range (start (line 27) (character 5)) (end (line 27) (character 108))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (kind "item def") (name "StateSpaceItem") (declared-name "StateSpaceItem") (range (start (line 35) (character 4)) (end (line 35) (character 93))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem::_documentation"))) (kind "documentation") (name "") (range (start (line 35) (character 4)) (end (line 35) (character 93))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 8) (character 4)) (end (line 8) (character 51))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 50))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind "action def") (name "ZeroCrossingEventDef") (declared-name "ZeroCrossingEventDef") (range (start (line 33) (character 4)) (end (line 33) (character 58))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceEventDef") (range none)) (specializes (reference "StateSpaceEventDef") (range none)) (specializes (reference "StateSpaceEventDef") (range (start (line 33) (character 39)) (end (line 33) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4263))) (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VectorCalculations::*") (range (start (line 9) (character 19)) (end (line 9) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceDynamics") (range (start (line 93) (character 56)) (end (line 93) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceDynamics") (range (start (line 129) (character 54)) (end (line 129) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (range (start (line 7) (character 19)) (end (line 7) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateDerivative") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "GetDerivative") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range (start (line 64) (character 23)) (end (line 64) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (range (start (line 8) (character 19)) (end (line 8) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceEventDef") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceEventDef") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceEventDef") (range (start (line 33) (character 39)) (end (line 33) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 2)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 64 23) (end 64 34)) (probe (position 64 23))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))
        (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
        (range (start 64 23) (end 64 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace") (range (start 11 4) (end 11 61)))
        )
      )
    )
    (query (range (start 7 19) (end 7 37)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
        (range (start 7 19) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 37)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VectorCalculations::*")
        (range (start 9 19) (end 9 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 39) (end 33 57)) (probe (position 33 39))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))
        (kind specialization) (ordinal 2) (authored-target "StateSpaceEventDef")
        (range (start 33 39) (end 33 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef") (range (start 27 5) (end 27 108)))
        )
      )
    )
    (query (range (start 93 56) (end 93 74)) (probe (position 93 56))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))
        (kind specialization) (ordinal 2) (authored-target "StateSpaceDynamics")
        (range (start 93 56) (end 93 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics") (range (start 42 4) (end 42 480)))
        )
      )
    )
    (query (range (start 129 54) (end 129 72)) (probe (position 129 54))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))
        (kind specialization) (ordinal 2) (authored-target "StateSpaceDynamics")
        (range (start 129 54) (end 129 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics") (range (start 42 4) (end 42 480)))
        )
      )
    )
    (query (range (start 8 19) (end 8 50)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
        (range (start 8 19) (end 8 50))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
