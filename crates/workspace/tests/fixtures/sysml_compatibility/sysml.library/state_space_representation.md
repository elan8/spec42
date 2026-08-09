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
    doc /*
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
        in input : Input;
        in stateSpace : StateSpace;
        in timeStep : DurationValue;
        return : StateSpace;
    }
    abstract calc def GetOutput {
        in input : Input;
        in stateSpace : StateSpace;
        return : Output;
    }

    abstract action def StateSpaceEventDef {
        doc /*
    	 * Events to be received.
    	 */
    }
    action def ZeroCrossingEventDef :> StateSpaceEventDef;

    item def StateSpaceItem {
        doc /*
    	 * Item for SSR connection
    	 */
    }

    abstract action def StateSpaceDynamics {
        doc /*
	     * StateSpaceDynamics is the simplest form of State Space Representation,
	     * and nextState directly computes the stateSpace of the next timestep. 
	     */

        in attribute input : Input;

        abstract calc getNextState : GetNextState;
        abstract calc getOutput : GetOutput;
        attribute stateSpace : StateSpace;

        out attribute output : Output = getOutput(input, stateSpace);
    }

    abstract attribute def StateDerivative :> VectorQuantityValue {
        doc /*
	     * The definition of the time derivative of StateSpace, which means dx/dt, where x is StateSpace
	     */

        ref stateSpace : StateSpace;
        constraint {
            = stateSpace.order == order;
        }
    }

    abstract calc def GetDerivative {
        doc /*
	     * Computes the time derivative of stateSpace, which corresponds dx/dt = f(u, x), where u is input and x is stateSpace.
	     */

        in input : Input;
        in stateSpace : StateSpace;
        return : StateDerivative;
    }

    abstract calc def Integrate {
        doc /*
	     * Integrates stateSpace to compute the next stateSpace, which corresponds to x + int dx/dt dt.
	     * Its actual implementation should be given by a solver. 
	     */

        in getDerivative : GetDerivative;
        in input : Input;
        in initialState : StateSpace;
        in timeInterval : DurationValue;
        return result: StateSpace;
    }

    abstract action def ContinuousStateSpaceDynamics :> StateSpaceDynamics {
        doc /*
	     * ContinuousStateSpaceDynamics represents continuous behavior.
	     * derivative needs to return a time derivative of stateSpace, i.e. dx/dt.
	     */

        abstract calc getDerivative : GetDerivative;
        calc :>> getNextState : GetNextState {
            /* We compute nextState by Integrate defined above by giving derivative calc. */
            calc integrate : Integrate {
                in getDerivative = ContinuousStateSpaceDynamics::getDerivative;
                in input = GetNextState::input;
                in initialState = GetNextState::stateSpace;
                in timeInterval = GetNextState::timeStep;
            }
            return result = integrate.result;
        }

        event occurrence zeroCrossingEvents : ZeroCrossingEventDef [0..*] {
            /* ContinuousStateSpaceDynamics may cause zero crossings anomaly. */
        }
    }

    abstract calc def GetDifference {
        doc /*
	     * Computes difference of stateSpace by one timestep, that is x(k+1) - x(k),
	     * where k is the timestep number. 
	     */

        in input : Input;
        in stateSpace : StateSpace;
        return : StateSpace;
    }

    abstract action def DiscreteStateSpaceDynamics :> StateSpaceDynamics {
        doc /*
	     * DiscreteStateSpaceDynamics represents discrete behavior.
	     * differences needs to return difference of the stateSpace for each timestep.
	     */

        abstract calc getDifference : GetDifference;
        calc :>> getNextState : GetNextState {
            attribute diff : StateSpace = getDifference(input, stateSpace);
            return result = stateSpace + diff;
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'StateSpaceRepresentation'
      (documentation)
      (membership_import private -> 'ISQ::DurationValue'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (namespace_import private -> 'VectorCalculations'[unresolved])
      (attribute_def abstract 'StateSpace' :> 'VectorQuantityValue'[unresolved])
      (attribute_def abstract 'Input' :> 'VectorQuantityValue'[unresolved])
      (attribute_def abstract 'Output' :> 'VectorQuantityValue'[unresolved])
      (calculation_def abstract 'GetNextState'
        (reference_usage in reference 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (reference_usage in reference 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (reference_usage in reference 'timeStep' : 'DurationValue'[unresolved])
        (return_parameter_membership
          (feature_def out : 'StateSpaceRepresentation::StateSpace'[attribute_def])))
      (calculation_def abstract 'GetOutput'
        (reference_usage in reference 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (reference_usage in reference 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'StateSpaceRepresentation::Output'[attribute_def])))
      (action_def abstract 'StateSpaceEventDef'
        (documentation))
      (action_def 'ZeroCrossingEventDef' :> 'StateSpaceRepresentation::StateSpaceEventDef'[action_def])
      (item_def 'StateSpaceItem'
        (documentation))
      (action_def abstract 'StateSpaceDynamics'
        (documentation)
        (attribute_usage in 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (calculation_usage abstract composite 'getNextState' : 'StateSpaceRepresentation::GetNextState'[calculation_def])
        (calculation_usage abstract composite 'getOutput' : 'StateSpaceRepresentation::GetOutput'[calculation_def])
        (attribute_usage composite 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (attribute_usage out 'output' : 'StateSpaceRepresentation::Output'[attribute_def]
          (feature_value (=))))
      (attribute_def abstract 'StateDerivative' :> 'VectorQuantityValue'[unresolved]
        (documentation)
        (reference_usage reference 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (constraint_usage composite
          (result_expr_membership)))
      (calculation_def abstract 'GetDerivative'
        (documentation)
        (reference_usage in reference 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (reference_usage in reference 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'StateSpaceRepresentation::StateDerivative'[attribute_def])))
      (calculation_def abstract 'Integrate'
        (documentation)
        (reference_usage in reference 'getDerivative' : 'StateSpaceRepresentation::GetDerivative'[calculation_def])
        (reference_usage in reference 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (reference_usage in reference 'initialState' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (reference_usage in reference 'timeInterval' : 'DurationValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'result' : 'StateSpaceRepresentation::StateSpace'[attribute_def])))
      (action_def abstract 'ContinuousStateSpaceDynamics' :> 'StateSpaceRepresentation::StateSpaceDynamics'[action_def]
        (documentation)
        (calculation_usage abstract composite 'getDerivative' : 'StateSpaceRepresentation::GetDerivative'[calculation_def])
        (calculation_usage composite :>> 'StateSpaceRepresentation::StateSpaceDynamics::getNextState'[calculation_usage] : 'StateSpaceRepresentation::GetNextState'[calculation_def]
          (calculation_usage composite 'integrate' : 'StateSpaceRepresentation::Integrate'[calculation_def]
            (reference_usage in reference 'getDerivative'
              (feature_value (=)))
            (reference_usage in reference 'input'
              (feature_value (=)))
            (reference_usage in reference 'initialState'
              (feature_value (=)))
            (reference_usage in reference 'timeInterval'
              (feature_value (=))))
          (return_parameter_membership
            (feature_def out 'result'
              (feature_value (=)))))
        (event_occurrence_usage 'zeroCrossingEvents' : 'StateSpaceRepresentation::ZeroCrossingEventDef'[action_def]
          (multiplicity_range [0..*])))
      (calculation_def abstract 'GetDifference'
        (documentation)
        (reference_usage in reference 'input' : 'StateSpaceRepresentation::Input'[attribute_def])
        (reference_usage in reference 'stateSpace' : 'StateSpaceRepresentation::StateSpace'[attribute_def])
        (return_parameter_membership
          (feature_def out : 'StateSpaceRepresentation::StateSpace'[attribute_def])))
      (action_def abstract 'DiscreteStateSpaceDynamics' :> 'StateSpaceRepresentation::StateSpaceDynamics'[action_def]
        (documentation)
        (calculation_usage abstract composite 'getDifference' : 'StateSpaceRepresentation::GetDifference'[calculation_def])
        (calculation_usage composite :>> 'StateSpaceRepresentation::StateSpaceDynamics::getNextState'[calculation_usage] : 'StateSpaceRepresentation::GetNextState'[calculation_def]
          (attribute_usage composite 'diff' : 'StateSpaceRepresentation::StateSpace'[attribute_def]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'result'
              (feature_value (=)))))))))
~~~
