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
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (kind "package") (name "StateSpaceRepresentation") (declared-name "StateSpaceRepresentation"))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorCalculations::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind "action def") (name "ContinuousStateSpaceDynamics") (declared-name "ContinuousStateSpaceDynamics") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceDynamics")) (specializes (reference "StateSpaceDynamics")) (specializes (reference "StateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics:::>> getNextState: GetNextState"))) (kind "action body decl") (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::getDerivative: GetDerivative"))) (kind "action body decl") (name "getDerivative: GetDerivative") (declared-name "getDerivative: GetDerivative") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef"))) (kind "action body decl") (name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (declared-name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind "action def") (name "DiscreteStateSpaceDynamics") (declared-name "DiscreteStateSpaceDynamics") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceDynamics")) (specializes (reference "StateSpaceDynamics")) (specializes (reference "StateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics:::>> getNextState: GetNextState"))) (kind "action body decl") (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::getDifference: GetDifference"))) (kind "action body decl") (name "getDifference: GetDifference") (declared-name "getDifference: GetDifference") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (kind "calc def") (name "GetDerivative") (declared-name "GetDerivative") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "StateDerivative")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (kind "calc def") (name "GetDifference") (declared-name "GetDifference") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (kind "calc def") (name "GetNextState") (declared-name "GetNextState") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind "in out parameter") (name "timeStep") (declared-name "timeStep") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (authored (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (kind "calc def") (name "GetOutput") (declared-name "GetOutput") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind "in out parameter") (name "stateSpace") (declared-name "stateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (kind "attribute def") (name "Input") (declared-name "Input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (kind "calc def") (name "Integrate") (declared-name "Integrate") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind "in out parameter") (name "getDerivative") (declared-name "getDerivative") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "GetDerivative")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind "in out parameter") (name "initialState") (declared-name "initialState") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind "return parameter") (name "result") (declared-name "result") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind "in out parameter") (name "timeInterval") (declared-name "timeInterval") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (authored (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (kind "attribute def") (name "Output") (declared-name "Output") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind "attribute def") (name "StateDerivative") (declared-name "StateDerivative") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind "ref") (name "stateSpace") (declared-name "stateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind "attribute def") (name "StateSpace") (declared-name "StateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (kind "action def") (name "StateSpaceDynamics") (declared-name "StateSpaceDynamics") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getNextState: GetNextState"))) (kind "action body decl") (name "getNextState: GetNextState") (declared-name "getNextState: GetNextState") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getOutput: GetOutput"))) (kind "action body decl") (name "getOutput: GetOutput") (declared-name "getOutput: GetOutput") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (authored (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (authored (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::stateSpace: StateSpace"))) (kind "action body decl") (name "stateSpace: StateSpace") (declared-name "stateSpace: StateSpace") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (kind "action def") (name "StateSpaceEventDef") (declared-name "StateSpaceEventDef") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (kind "item def") (name "StateSpaceItem") (declared-name "StateSpaceItem") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind "action def") (name "ZeroCrossingEventDef") (declared-name "ZeroCrossingEventDef") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateSpaceEventDef")) (specializes (reference "StateSpaceEventDef")) (specializes (reference "StateSpaceEventDef")))))
    (element (id (node (document "d0") (qualified-name "StateSpaceRepresentation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "StateSpaceRepresentation"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VectorCalculations::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceDynamics") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateDerivative") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "GetDerivative") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Input")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::Output")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 0)) (authored-target "StateSpaceEventDef") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 1)) (authored-target "StateSpaceEventDef") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
    (reference (id (source (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (kind specialization) (ordinal 2)) (authored-target "StateSpaceEventDef") (outcome (status resolved) (target (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
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
