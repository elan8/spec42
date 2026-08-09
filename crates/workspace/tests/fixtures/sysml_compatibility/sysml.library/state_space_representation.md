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
# EXPECTED
~~~
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'DurationValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'DurationValue'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwAbstract,KwCalc,Ident,Colon,Ident,Semicolon,
KwAbstract,KwCalc,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,Semicolon,
KwConstraint,OpenCurly,Ident,Dot,Ident,EqEq,Ident,CloseCurly,
CloseCurly,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwCalc,Ident,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwReturn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwEvent,KwOccurrence,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwCalc,Ident,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwReturn,Ident,Eq,Ident,Plus,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'StateSpaceRepresentation'
    (documentation)
    (import_decl private 'ISQ::DurationValue')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'VectorCalculations::*')
    (attribute_def abstract 'StateSpace' :> 'VectorQuantityValue')
    (attribute_def abstract 'Input' :> 'VectorQuantityValue')
    (attribute_def abstract 'Output' :> 'VectorQuantityValue')
    (calc_def abstract 'GetNextState'
      (default_ref_usage in 'input' : 'Input')
      (default_ref_usage in 'stateSpace' : 'StateSpace')
      (default_ref_usage in 'timeStep' : 'DurationValue')
      (return_member))
    (calc_def abstract 'GetOutput'
      (default_ref_usage in 'input' : 'Input')
      (default_ref_usage in 'stateSpace' : 'StateSpace')
      (return_member))
    (action_def abstract 'StateSpaceEventDef'
      (documentation))
    (action_def 'ZeroCrossingEventDef' :> 'StateSpaceEventDef')
    (item_def 'StateSpaceItem'
      (documentation))
    (action_def abstract 'StateSpaceDynamics'
      (documentation)
      (attribute_usage in 'input' : 'Input')
      (calc_usage abstract 'getNextState' : 'GetNextState')
      (calc_usage abstract 'getOutput' : 'GetOutput')
      (attribute_usage 'stateSpace' : 'StateSpace')
      (attribute_usage out 'output' : 'Output' value))
    (attribute_def abstract 'StateDerivative' :> 'VectorQuantityValue'
      (documentation)
      (ref_usage ref 'stateSpace' : 'StateSpace')
      (constraint_usage
        (result_expr_member)))
    (calc_def abstract 'GetDerivative'
      (documentation)
      (default_ref_usage in 'input' : 'Input')
      (default_ref_usage in 'stateSpace' : 'StateSpace')
      (return_member))
    (calc_def abstract 'Integrate'
      (documentation)
      (default_ref_usage in 'getDerivative' : 'GetDerivative')
      (default_ref_usage in 'input' : 'Input')
      (default_ref_usage in 'initialState' : 'StateSpace')
      (default_ref_usage in 'timeInterval' : 'DurationValue')
      (return_member))
    (action_def abstract 'ContinuousStateSpaceDynamics' :> 'StateSpaceDynamics'
      (documentation)
      (calc_usage abstract 'getDerivative' : 'GetDerivative')
      (calc_usage :>> 'getNextState' : 'GetNextState'
        (comment)
        (calc_usage 'integrate' : 'Integrate'
          (default_ref_usage in 'getDerivative' value)
          (default_ref_usage in 'input' value)
          (default_ref_usage in 'initialState' value)
          (default_ref_usage in 'timeInterval' value))
        (return_member))
      (event_occurrence 'zeroCrossingEvents' : 'ZeroCrossingEventDef' multiplicity
        (comment)))
    (calc_def abstract 'GetDifference'
      (documentation)
      (default_ref_usage in 'input' : 'Input')
      (default_ref_usage in 'stateSpace' : 'StateSpace')
      (return_member))
    (action_def abstract 'DiscreteStateSpaceDynamics' :> 'StateSpaceDynamics'
      (documentation)
      (calc_usage abstract 'getDifference' : 'GetDifference')
      (calc_usage :>> 'getNextState' : 'GetNextState'
        (attribute_usage 'diff' : 'StateSpace' value)
        (return_member)))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (name "StateSpaceRepresentation") (declared-name "StateSpaceRepresentation")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::*"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (name "ContinuousStateSpaceDynamics") (declared-name "ContinuousStateSpaceDynamics")
          (contains
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics:::>> getNextState: GetNextState"))) (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::getDerivative: GetDerivative"))) (name "getDerivative: GetDerivative") (declared-name "getDerivative: GetDerivative") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef"))) (name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (declared-name "occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (name "DiscreteStateSpaceDynamics") (declared-name "DiscreteStateSpaceDynamics")
          (contains
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics:::>> getNextState: GetNextState"))) (name ":>> getNextState: GetNextState") (declared-name ":>> getNextState: GetNextState") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::getDifference: GetDifference"))) (name "getDifference: GetDifference") (declared-name "getDifference: GetDifference") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::DurationValue"))) (name "DurationValue") (declared-name "DurationValue"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (name "GetDerivative") (declared-name "GetDerivative")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (name "stateSpace") (declared-name "stateSpace") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (name "GetDifference") (declared-name "GetDifference")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (name "stateSpace") (declared-name "stateSpace") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (name "GetNextState") (declared-name "GetNextState")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (name "stateSpace") (declared-name "stateSpace") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::timeStep"))) (name "timeStep") (declared-name "timeStep") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (name "GetOutput") (declared-name "GetOutput")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (name "stateSpace") (declared-name "stateSpace") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (name "Input") (declared-name "Input") (declared (properties (ordered false) (unique true))))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (name "Integrate") (declared-name "Integrate")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (name "getDerivative") (declared-name "getDerivative") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (name "initialState") (declared-name "initialState") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (name "result") (declared-name "result") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::timeInterval"))) (name "timeInterval") (declared-name "timeInterval") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (name "Output") (declared-name "Output") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (name "StateDerivative") (declared-name "StateDerivative") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (name "stateSpace") (declared-name "stateSpace") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (name "StateSpace") (declared-name "StateSpace") (declared (properties (ordered false) (unique true))))
        (element (kind "action def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (name "StateSpaceDynamics") (declared-name "StateSpaceDynamics")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getNextState: GetNextState"))) (name "getNextState: GetNextState") (declared-name "getNextState: GetNextState") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::getOutput: GetOutput"))) (name "getOutput: GetOutput") (declared-name "getOutput: GetOutput") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (name "input") (declared-name "input") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (name "output") (declared-name "output") (declared (properties (direction "out")) (own-expression (expression (kind "invocation") (children (expression (kind "featureReference") (reference "getOutput"))) (arguments (argument (expression (kind "featureReference") (reference "input"))) (argument (expression (kind "featureReference") (reference "stateSpace"))))))) (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::stateSpace: StateSpace"))) (name "stateSpace: StateSpace") (declared-name "stateSpace: StateSpace") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (name "StateSpaceEventDef") (declared-name "StateSpaceEventDef")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (name "StateSpaceItem") (declared-name "StateSpaceItem")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (name "ZeroCrossingEventDef") (declared-name "ZeroCrossingEventDef"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "StateSpaceRepresentation::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::_documentation"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative::stateSpace"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference::stateSpace"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState::stateSpace"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput::stateSpace"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::getDerivative"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::initialState"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate::result"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative::stateSpace"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::input"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics::output"))) (to (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::ContinuousStateSpaceDynamics"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::DiscreteStateSpaceDynamics"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDerivative"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetDifference"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetNextState"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::GetOutput"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Input"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Integrate"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::Output"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateDerivative"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpace"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceDynamics"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceEventDef"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::StateSpaceItem"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "StateSpaceRepresentation::ZeroCrossingEventDef"))) (status missing-prerequisite) (target "Actions::Action"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/state_space_representation.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 4) (end 11 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 4) (end 12 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 4) (end 13 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 5) (end 18 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 4) (end 58 291))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 8) (end 89 39))
      )
    )
  )
)
~~~
