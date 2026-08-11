# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Triggers
type=file
~~~
# SOURCE
~~~kerml
standard library package Triggers {
	doc
	/*
	 * This package contains functions that return ChangeSignals for triggering
	 * when a Boolean condition changes from false to true, at a specific time
	 * or after a specific time delay.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::NumericalValue;
	private import Occurrences::Occurrence;
	
	public import Clocks::*;
	public import Observation::*;
	
	struct TimeSignal :> ChangeSignal {
		doc
		/*
		 * A TimeSignal is a ChangeSignal whose condition is the currentTime
		 * of a given Clock reaching a specific signalTime.
		 */
	
		feature signalTime : NumericalValue[1] {
			doc
			/*
			 * The time at which the TimeSignal should be sent.
			 */
		}
		
		feature signalClock : Clock[1] {
			doc
			/*
			 * The Clock whose currentTime is being monitored.
			 */
		}
		
		private bool :>> signalCondition {
			doc
			/*
			 * The Boolean condition of the currentTime of the signalClock being
			 * equal to the signalTime.
			 */
		
			signalClock.currentTime == signalTime
		}
	}
	
	function TriggerWhen {
		doc
		/*
		 * TriggerWhen returns a monitored ChangeSignal for a given condition,
		 * to be sent to a given receiver when the condition occurs.
		 */
	
		in bool condition[1] {
			doc
			/*
			 * The BooleanExpression to be monitored for changing from 
			 * false to true.
		 */
		}
		
		in feature receiver : Occurrence [1] {
			doc
			/*
			 * The Occurrence to which the ChangeSignal is to be sent.
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the ChangeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature changeSignal : ChangeSignal[1] = new ChangeSignal(condition, monitor) {
			doc
			/*
			 * The ChangeSignal for the condition, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = changeSignal;
		}		
	}
	
	function TriggerAt {
		doc
		/*
		 * TriggerAt returns a monitored TimeSignal to be sent to a receiver when
		 * the currentTime of a given Clock reaches a specific timeInstant. 
		 */
	
		in feature timeInstant : NumericalValue[1] {
			doc
			/*
			 * The time instant, relative to the clock, at which the TimeSignal should be sent. 
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the timeInstant. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature timeSignal : TimeSignal[1] = new TimeSignal(timeInstant, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the given timeInstant, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = timeSignal;
		}
	}
	
	function TriggerAfter {
		doc
		/*
		 * TriggerAfter returns a monitored TimeSignal to be sent to a receiver after
		 * a certain time delay relative to a given Clock.
		 */
	
		in feature delay : NumericalValue[1] {
			doc
			/*
			 * The time duration, relative to the clock, after which the TimeSignal is sent.
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the time delay. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return signal : TimeSignal[1] = 
			TriggerAt(clock.currentTime + delay, receiver, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the currentTime of the clock when the function is invoked
			 * plus the given time delay, as monitored by the monitor.
			 */
		}
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "triggers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 15) (end 12 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 15) (end 13 26))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwStruct,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwBool,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Dot,Ident,EqEq,Ident,
CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwBool,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwReturn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Ident,OpenParen,Ident,Comma,Ident,CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwStep,ColonGt,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwReturn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwStep,ColonGt,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,Dot,Ident,Plus,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Triggers'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl public 'Clocks::*')
    (import_decl public 'Observation::*')
    (structure_def 'TimeSignal' :> 'ChangeSignal'
      (documentation)
      (feature_def 'signalTime' : 'NumericalValue' multiplicity
        (documentation))
      (feature_def 'signalClock' : 'Clock' multiplicity
        (documentation))
      (boolean_expr_def
        (documentation)
        (result_expr_member)))
    (function_def
      (documentation)
      (boolean_expr_usage
        (documentation))
      (feature_def in 'receiver' : 'Occurrence' multiplicity
        (documentation))
      (feature_def in 'monitor' : 'ChangeMonitor' multiplicity value
        (documentation))
      (return_member)
      (step_def
        (feature_def in 'observer' value)
        (feature_def in 'signal' value)))
    (function_def
      (documentation)
      (feature_def in 'timeInstant' : 'NumericalValue' multiplicity
        (documentation))
      (feature_def in 'receiver' : 'Occurrence' multiplicity
        (documentation))
      (feature_def in 'clock' : 'Clock' multiplicity value
        (documentation))
      (feature_def in 'monitor' : 'ChangeMonitor' multiplicity value
        (documentation))
      (return_member)
      (step_def
        (feature_def in 'observer' value)
        (feature_def in 'signal' value)))
    (function_def
      (documentation)
      (feature_def in 'delay' : 'NumericalValue' multiplicity
        (documentation))
      (feature_def in 'receiver' : 'Occurrence' multiplicity
        (documentation))
      (feature_def in 'clock' : 'Clock' multiplicity value
        (documentation))
      (feature_def in 'monitor' : 'ChangeMonitor' multiplicity value
        (documentation))
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ChangeSignal'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'signalCondition'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'ChangeMonitor'
semantic.unresolved_name 'ChangeSignal'
semantic.unresolved_name 'monitor::startObservation'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ChangeMonitor'
semantic.unresolved_name 'monitor::startObservation'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ChangeMonitor'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ChangeSignal'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'signalCondition'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'ChangeMonitor'
semantic.unresolved_name 'ChangeSignal'
semantic.unresolved_name 'monitor::startObservation'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ChangeMonitor'
semantic.unresolved_name 'monitor::startObservation'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ChangeMonitor'
~~~
# FORMAT
~~~sysml
standard library package Triggers {
	doc
	/*
	 * This package contains functions that return ChangeSignals for triggering
	 * when a Boolean condition changes from false to true, at a specific time
	 * or after a specific time delay.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::NumericalValue;
	private import Occurrences::Occurrence;
	
	public import Clocks::*;
	public import Observation::*;
	
	struct TimeSignal :> ChangeSignal {
		doc
		/*
		 * A TimeSignal is a ChangeSignal whose condition is the currentTime
		 * of a given Clock reaching a specific signalTime.
		 */
	
		feature signalTime : NumericalValue[1] {
			doc
			/*
			 * The time at which the TimeSignal should be sent.
			 */
		}
		
		feature signalClock : Clock[1] {
			doc
			/*
			 * The Clock whose currentTime is being monitored.
			 */
		}
		
		private bool :>> signalCondition {
			doc
			/*
			 * The Boolean condition of the currentTime of the signalClock being
			 * equal to the signalTime.
			 */
		
			signalClock.currentTime == signalTime
		}
	}
	
	function TriggerWhen {
		doc
		/*
		 * TriggerWhen returns a monitored ChangeSignal for a given condition,
		 * to be sent to a given receiver when the condition occurs.
		 */
	
		in bool condition[1] {
			doc
			/*
			 * The BooleanExpression to be monitored for changing from 
			 * false to true.
		 */
		}
		
		in feature receiver : Occurrence [1] {
			doc
			/*
			 * The Occurrence to which the ChangeSignal is to be sent.
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the ChangeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature changeSignal : ChangeSignal[1] = new ChangeSignal(condition, monitor) {
			doc
			/*
			 * The ChangeSignal for the condition, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = changeSignal;
		}		
	}
	
	function TriggerAt {
		doc
		/*
		 * TriggerAt returns a monitored TimeSignal to be sent to a receiver when
		 * the currentTime of a given Clock reaches a specific timeInstant. 
		 */
	
		in feature timeInstant : NumericalValue[1] {
			doc
			/*
			 * The time instant, relative to the clock, at which the TimeSignal should be sent. 
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the timeInstant. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature timeSignal : TimeSignal[1] = new TimeSignal(timeInstant, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the given timeInstant, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = timeSignal;
		}
	}
	
	function TriggerAfter {
		doc
		/*
		 * TriggerAfter returns a monitored TimeSignal to be sent to a receiver after
		 * a certain time delay relative to a given Clock.
		 */
	
		in feature delay : NumericalValue[1] {
			doc
			/*
			 * The time duration, relative to the clock, after which the TimeSignal is sent.
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the time delay. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return signal : TimeSignal[1] = 
			TriggerAt(clock.currentTime + delay, receiver, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the currentTime of the clock when the function is invoked
			 * plus the given time delay, as monitored by the monitor.
			 */
		}
	}	
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "870c469b959364a0b9d42a667902528e5670c07da6322b297c8d7b599f7fd8ca") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Triggers"))) (kind "package") (name "Triggers") (declared-name "Triggers") (range (start (line 0) (character 0)) (end (line 0) (character 4422))))
    (element (id (node (document "d0") (qualified-name "Triggers::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 1)) (end (line 12) (character 25))) (parent (node (document "d0") (qualified-name "Triggers"))) (authored (membership (kind Import) (visibility "public") (import (reference "Clocks::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 15)) (end (line 12) (character 21))))))
    (element (id (node (document "d0") (qualified-name "Triggers::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 13) (character 1)) (end (line 13) (character 30))) (parent (node (document "d0") (qualified-name "Triggers"))) (authored (membership (kind Import) (visibility "public") (import (reference "Observation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 13) (character 15)) (end (line 13) (character 26))))))
    (element (id (node (document "d0") (qualified-name "Triggers::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 8) (character 1)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Triggers::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 9) (character 1)) (end (line 9) (character 45))) (parent (node (document "d0") (qualified-name "Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Triggers::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Triggers::TimeSignal"))) (kind "classifier decl") (name "TimeSignal") (declared-name "TimeSignal") (range (start (line 15) (character 1)) (end (line 15) (character 632))) (parent (node (document "d0") (qualified-name "Triggers"))))
    (element (id (node (document "d0") (qualified-name "Triggers::TriggerAfter"))) (kind "kermlDecl") (name "TriggerAfter") (declared-name "TriggerAfter") (range (start (line 140) (character 1)) (end (line 140) (character 1178))) (parent (node (document "d0") (qualified-name "Triggers"))))
    (element (id (node (document "d0") (qualified-name "Triggers::TriggerAt"))) (kind "kermlDecl") (name "TriggerAt") (declared-name "TriggerAt") (range (start (line 90) (character 1)) (end (line 90) (character 1219))) (parent (node (document "d0") (qualified-name "Triggers"))))
    (element (id (node (document "d0") (qualified-name "Triggers::TriggerWhen"))) (kind "kermlDecl") (name "TriggerWhen") (declared-name "TriggerWhen") (range (start (line 47) (character 1)) (end (line 47) (character 952))) (parent (node (document "d0") (qualified-name "Triggers"))))
    (element (id (node (document "d0") (qualified-name "Triggers::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4422))) (parent (node (document "d0") (qualified-name "Triggers"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Triggers::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Clocks::*") (range (start (line 12) (character 15)) (end (line 12) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Triggers::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Observation::*") (range (start (line 13) (character 15)) (end (line 13) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Triggers::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Triggers::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (range (start (line 9) (character 16)) (end (line 9) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Triggers::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
