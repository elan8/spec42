# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Clocks
type=file
~~~
# SOURCE
~~~kerml
standard library package Clocks {
	doc
	/*
	 * This package models Clocks that provide an advancing numerical reference 
	 * usable for quantifying the time of an Occurrence.
	 */

	private import ScalarValues::NumericalValue;
	private import ScalarValues::Real;
	private import Occurrences::Occurrence;
	private import Occurrences::Life;
	private import ControlFunctions::forAll;
	
	private struct UniversalClockLife[1] :> Clock, Life {
	    doc
	    /*
	     * UniversalClockLife is the classifier of the singleton Life of the universalClock.
	     */
	}
	
	feature universalClock : UniversalClockLife[1] {
		doc
		/*
		 * universalClock is a single Clock that can be used as a default universal
		 * time reference.
		 */
	}
	
	abstract struct Clock {
		doc
		/*
		 * A Clock provides a numerical currentTime that advances montonically
		 * over its lifetime. Clock is an abstract base Structure that can be
		 * specialized for different kinds of time quantification (e.g., discrete
		 * time, continuous time, time with units, etc.).
		 */
		 
		private thisClock : Clock :>> self;
		
		var feature currentTime : NumericalValue[1] {
			doc
			/*
			 * A scalar time reference that advances over the lifetime of the Clock. 
			 */
		}
						
		inv timeFlowConstraint {
			doc
			/*
			 * The currentTime of a snapshot of a Clock is equal to
			 * the TimeOf the snapshot relative to that Clock.
			 */
			
			snapshots->forAll{in s : Clock; 
				TimeOf(s, thisClock) == s.currentTime
			}
		}		
	}
	
	abstract function TimeOf {
		doc
		/*
		 * TimeOf returns a numerical timeInstant for a given Occurrence relative to
		 * a given Clock. The timeInstant is the time of the start of the Occurrence,
		 * which is considered to be synchronized with the snapshot of the Clock 
		 * with a currentTime equal to the returned timeInstant.
		 */
		
		in o : Occurrence[1];
		in clock : Clock[1] default localClock;
		return timeInstant : NumericalValue[1];
		
		 inv startTimeConstraint {
		 	doc
			/*
			 * The TimeOf an Occurrence is equal to the time of its start snapshot.
			 */
			 
		 	timeInstant == TimeOf(o.startShot, clock)
		 }	 

		inv timeOrderingConstraint {
			doc
			/*
			 * If one Occurrence happens before another, then the TimeOf the end
			 * snapshot of the first Occurrence is no greater than the TimeOf the 
			 * second Occurrence.
			 */
			
			o.predecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) <= timeInstant
			}
		}
				
		inv timeContinuityConstraint {
			doc
			/*
			 * If one Occurrence happens immediately before another, then the TimeOf 
			 * the end snapshot of the first Occurrence equals the TimeOf the second
			 * Occurrence.
			 */
		 
			o.immediatePredecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) == timeInstant
			}
		}				
	}
	
	function DurationOf {
		doc
		/*
		 * DurationOf returns the duration of a given Occurrence relative to a
		 * given Clock, which is equal to the TimeOf the end snapshot of the
		 * Occurrence minus the TimeOf its start snapshot.
		 */
		
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return duration : NumericalValue =
			TimeOf(o.endShot, clock) - TimeOf(o.startShot, clock);
	}
	
	struct BasicClock :> Clock {
		doc
		/*
		 * A BasicClock is a Clock whose currentTime is a Real number.
		 */
		
		var feature :>> currentTime : Real;
	}
	
	function BasicTimeOf :> TimeOf {
		doc
		/*
		 * BasicTimeOf returns the TimeOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */

		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}
	
	function BasicDurationOf :> DurationOf {
		doc
		/*
		 * BasicDurationOf returns the DurationOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */
		
		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'self'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'self'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwStruct,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwStruct,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,Comma,Ident,CloseParen,EqEq,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,EqEq,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,
CloseCurly,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,LtEq,Ident,
CloseCurly,
CloseCurly,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,EqEq,Ident,
CloseCurly,
CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,
Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,Minus,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwStruct,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwVar,KwFeature,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Clocks'
    (documentation)
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::Life')
    (import_decl private 'ControlFunctions::forAll')
    (structure_def private 'UniversalClockLife' multiplicity     (multiplicity_range) :> 'Clock', 'Life'
      (documentation))
    (feature_def 'universalClock' : 'UniversalClockLife' multiplicity
      (documentation))
    (structure_def abstract 'Clock'
      (documentation)
      (feature_def private 'thisClock' : 'Clock' :>> 'self')
      (feature_def var 'currentTime' : 'NumericalValue' multiplicity
        (documentation))
      (invariant_def
        (documentation)
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'o' : 'Occurrence' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member)
      (invariant_def
        (documentation)
        (result_expr_member))
      (invariant_def
        (documentation)
        (result_expr_member))
      (invariant_def
        (documentation)
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'o' : 'Occurrence' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (structure_def 'BasicClock' :> 'Clock'
      (documentation)
      (feature_def var :>> 'currentTime' : 'Real'))
    (function_def
      (documentation)
      (feature_def in 'o' : 'Occurrence' multiplicity)
      (feature_def in 'clock' : 'BasicClock' multiplicity)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'o' : 'Occurrence' multiplicity)
      (feature_def in 'clock' : 'BasicClock' multiplicity)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package Clocks {
	doc
	/*
	 * This package models Clocks that provide an advancing numerical reference 
	 * usable for quantifying the time of an Occurrence.
	 */

	private import ScalarValues::NumericalValue;
	private import ScalarValues::Real;
	private import Occurrences::Occurrence;
	private import Occurrences::Life;
	private import ControlFunctions::forAll;
	
	private struct UniversalClockLife[1] :> Clock, Life {
	    doc
	    /*
	     * UniversalClockLife is the classifier of the singleton Life of the universalClock.
	     */
	}
	
	feature universalClock : UniversalClockLife[1] {
		doc
		/*
		 * universalClock is a single Clock that can be used as a default universal
		 * time reference.
		 */
	}
	
	abstract struct Clock {
		doc
		/*
		 * A Clock provides a numerical currentTime that advances montonically
		 * over its lifetime. Clock is an abstract base Structure that can be
		 * specialized for different kinds of time quantification (e.g., discrete
		 * time, continuous time, time with units, etc.).
		 */
		 
		private thisClock : Clock :>> self;
		
		var feature currentTime : NumericalValue[1] {
			doc
			/*
			 * A scalar time reference that advances over the lifetime of the Clock. 
			 */
		}
						
		inv timeFlowConstraint {
			doc
			/*
			 * The currentTime of a snapshot of a Clock is equal to
			 * the TimeOf the snapshot relative to that Clock.
			 */
			
			snapshots->forAll{in s : Clock; 
				TimeOf(s, thisClock) == s.currentTime
			}
		}		
	}
	
	abstract function TimeOf {
		doc
		/*
		 * TimeOf returns a numerical timeInstant for a given Occurrence relative to
		 * a given Clock. The timeInstant is the time of the start of the Occurrence,
		 * which is considered to be synchronized with the snapshot of the Clock 
		 * with a currentTime equal to the returned timeInstant.
		 */
		
		in o : Occurrence[1];
		in clock : Clock[1] default localClock;
		return timeInstant : NumericalValue[1];
		
		 inv startTimeConstraint {
		 	doc
			/*
			 * The TimeOf an Occurrence is equal to the time of its start snapshot.
			 */
			 
		 	timeInstant == TimeOf(o.startShot, clock)
		 }	 

		inv timeOrderingConstraint {
			doc
			/*
			 * If one Occurrence happens before another, then the TimeOf the end
			 * snapshot of the first Occurrence is no greater than the TimeOf the 
			 * second Occurrence.
			 */
			
			o.predecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) <= timeInstant
			}
		}
				
		inv timeContinuityConstraint {
			doc
			/*
			 * If one Occurrence happens immediately before another, then the TimeOf 
			 * the end snapshot of the first Occurrence equals the TimeOf the second
			 * Occurrence.
			 */
		 
			o.immediatePredecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) == timeInstant
			}
		}				
	}
	
	function DurationOf {
		doc
		/*
		 * DurationOf returns the duration of a given Occurrence relative to a
		 * given Clock, which is equal to the TimeOf the end snapshot of the
		 * Occurrence minus the TimeOf its start snapshot.
		 */
		
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return duration : NumericalValue =
			TimeOf(o.endShot, clock) - TimeOf(o.startShot, clock);
	}
	
	struct BasicClock :> Clock {
		doc
		/*
		 * A BasicClock is a Clock whose currentTime is a Real number.
		 */
		
		var feature :>> currentTime : Real;
	}
	
	function BasicTimeOf :> TimeOf {
		doc
		/*
		 * BasicTimeOf returns the TimeOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */

		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}
	
	function BasicDurationOf :> DurationOf {
		doc
		/*
		 * BasicDurationOf returns the DurationOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */
		
		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}

}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Clocks"))) (name "Clocks") (declared-name "Clocks")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Clocks::BasicClock"))) (name "BasicClock") (declared-name "BasicClock"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Clocks::BasicDurationOf"))) (name "BasicDurationOf") (declared-name "BasicDurationOf"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Clocks::BasicTimeOf"))) (name "BasicTimeOf") (declared-name "BasicTimeOf"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Clocks::Clock"))) (name "Clock") (declared-name "Clock"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Clocks::DurationOf"))) (name "DurationOf") (declared-name "DurationOf"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Clocks::Life"))) (name "Life") (declared-name "Life"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Clocks::NumericalValue"))) (name "NumericalValue") (declared-name "NumericalValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Clocks::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Clocks::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Clocks::TimeOf"))) (name "TimeOf") (declared-name "TimeOf"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Clocks::UniversalClockLife1"))) (name "UniversalClockLife1") (declared-name "UniversalClockLife1"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Clocks::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Clocks::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Clocks::universalClock"))) (name "universalClock") (declared-name "universalClock"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Clocks::_documentation"))) (to (node (document "d0") (qualified-name "Clocks"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/clocks.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 1) (end 11 41))
      )
    )
  )
)
~~~
