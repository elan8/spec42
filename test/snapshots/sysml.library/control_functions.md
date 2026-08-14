# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/ControlFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package ControlFunctions {
	doc
	/*
	 * This package defines functions that correspond to operators in the KerML expression notation 
	 * for which one or more operands are expressions whose evaluation is determined by another operand.
	 */

	private import Base::Anything;
	private import ScalarValues::ScalarValue;
	private import ScalarValues::Boolean;
	private import ScalarFunctions::min;
	private import ScalarFunctions::max;
	
	abstract function '.' {
		in feature source : Anything[0..*] nonunique {
	  		abstract feature target : Anything[0..*] nonunique;
	  	}	  	
	  	private feature chain chains source.target;
	    chain
	}
	
	abstract function 'if' { 
		in test: Boolean[1];
		in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }
		in expr elseValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function '??' {
		in firstValue: Anything[0..*] ordered nonunique;
		in expr secondValue[0..1] { return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function 'and' {
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'or'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function 'implies'{
		in firstValue: Boolean[1];
		in expr secondValue[0..1] { return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function collect { 
		in collection: Anything[0..*] ordered nonunique;
		in expr mapper[0..*] { in argument: Anything[1]; return : Anything[0..*] ordered nonunique; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function select { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr selector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	function selectOne { 
		in collection: Anything[0..*] ordered nonunique;
		in expr selector1[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..1] = collection->select {in x; selector1(x)}#(1);
	}
	
	abstract function reject{ 
		in collection: Anything[0..*] ordered nonunique; 
		in expr rejector[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function reduce { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr reducer[0..*] { in firstArg: Anything[1]; in secondArg: Anything[1]; return : Anything[1]; }
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function forAll { 
		in collection: Anything[0..*] ordered nonunique; 
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	abstract function exists { 
		in collection: Anything[0..*] ordered nonunique;
		in expr test[0..*] { in argument: Anything[1]; return : Boolean[1]; }
		return : Boolean[1];
	}
	
	function allTrue {
		in collection: Boolean[0..*]; 
		return : Boolean[1] = collection->forAll {in x; x};
	}
	
	function anyTrue {
		in collection: Boolean[0..*];
		return : Boolean[1] = collection->exists {in x; x};
	}
	
	function minimize {
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue[1] = collection->collect {in x; fn(x)}->reduce min;
	}
	
	function maximize { 
		in collection: ScalarValue[1..*];
		in expr fn[0..*] { in argument: ScalarValue[1]; return : ScalarValue[1]; }
		return : ScalarValue = collection->collect {in x; fn(x)}->reduce max;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/control_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 2) (end 16 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 5) (end 18 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 11) (end 22 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 23 2) (end 23 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 11) (end 25 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 17) (end 29 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 11) (end 31 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 17) (end 35 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 36 2) (end 36 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 11) (end 37 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 17) (end 41 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 42 2) (end 42 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 11) (end 43 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 17) (end 47 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 48 2) (end 48 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 11) (end 49 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 17) (end 53 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 54 2) (end 54 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 11) (end 55 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 17) (end 59 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 60 2) (end 60 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 11) (end 61 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 17) (end 65 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 66 2) (end 66 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 11) (end 67 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 17) (end 71 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 72 2) (end 72 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 11) (end 73 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 17) (end 77 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 78 2) (end 78 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 11) (end 79 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 17) (end 83 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 84 2) (end 84 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 11) (end 85 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 17) (end 89 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 90 2) (end 90 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 11) (end 91 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 17) (end 95 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 96 11) (end 96 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 17) (end 100 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 11) (end 101 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 17) (end 105 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 106 2) (end 106 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 107 11) (end 107 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 107 70) (end 107 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 17) (end 111 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 112 2) (end 112 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 11) (end 113 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 113 67) (end 113 70))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:d2293914690bae2b9666d1cf12a83b9bea4663d45fd5c59bc81f661f7d0dfd43") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::ScalarValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarFunctions::min") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarFunctions::max") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::."))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "chain"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::.::chain"))) (kind kerml-feature) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::??"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::??::firstValue"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (expressionOperand (reference "collection"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::and"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::and::firstValue"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (expressionOperand (reference "collection"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::collect"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::collect::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::exists"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::exists::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::forAll"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::forAll::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::if"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::if::test"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::implies"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::implies::firstValue"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue")) (expressionOperand (reference "collection")) (expressionOperand (reference "max"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue")) (expressionOperand (reference "collection")) (expressionOperand (reference "min"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::or"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::or::firstValue"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reduce"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reduce::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reject"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reject::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::select"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::select::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (expressionOperand (reference "collection"))))
    (declaration (id (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarFunctions::min")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarFunctions::max")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::."))) (kind expressionOperand) (ordinal 0))
      (authored-target "chain")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::??::firstValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection")))))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::and::firstValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection")))))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::collect::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::exists::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::forAll::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::if::test"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::implies::firstValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection")))))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "max")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection")))))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "min")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::or::firstValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reduce::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reject::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::select::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection")))))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::."))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/control_functions.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 8 16) (end 8 41)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 9 16) (end 9 37)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 10 16) (end 10 36)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarFunctions::min")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 11 16) (end 11 36)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarFunctions::max")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 18 5) (end 18 10)) (probe (position 18 5))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::."))) (kind expressionOperand) (ordinal 0) (authored-target "chain")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 31 11) (end 31 19)) (probe (position 31 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 29 17) (end 29 25)) (probe (position 29 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::??::firstValue"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 96 11) (end 96 18)) (probe (position 96 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 96 24) (end 96 34)) (probe (position 96 24))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection")))))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 95 17) (end 95 24)) (probe (position 95 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::allTrue::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 37 11) (end 37 18)) (probe (position 37 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 35 17) (end 35 24)) (probe (position 35 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::and::firstValue"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 101 11) (end 101 18)) (probe (position 101 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 101 24) (end 101 34)) (probe (position 101 24))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection")))))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 100 17) (end 100 24)) (probe (position 100 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::anyTrue::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 55 11) (end 55 19)) (probe (position 55 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 53 17) (end 53 25)) (probe (position 53 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::collect::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 91 11) (end 91 18)) (probe (position 91 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 89 17) (end 89 25)) (probe (position 89 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::exists::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 85 11) (end 85 18)) (probe (position 85 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 83 17) (end 83 25)) (probe (position 83 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::forAll::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 25 11) (end 25 19)) (probe (position 25 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 22 11) (end 22 18)) (probe (position 22 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::if::test"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 49 11) (end 49 18)) (probe (position 49 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 47 17) (end 47 24)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::implies::firstValue"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 113 11) (end 113 22)) (probe (position 113 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 113 25) (end 113 35)) (probe (position 113 25))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection")))))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 113 67) (end 113 70)) (probe (position 113 67))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "max")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 111 17) (end 111 28)) (probe (position 111 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::maximize::collection"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 107 11) (end 107 22)) (probe (position 107 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 107 28) (end 107 38)) (probe (position 107 28))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection")))))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 107 70) (end 107 73)) (probe (position 107 70))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "min")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 105 17) (end 105 28)) (probe (position 105 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::minimize::collection"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 43 11) (end 43 18)) (probe (position 43 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 41 17) (end 41 24)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::or::firstValue"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 79 11) (end 79 19)) (probe (position 79 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 77 17) (end 77 25)) (probe (position 77 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reduce::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 73 11) (end 73 19)) (probe (position 73 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 71 17) (end 71 25)) (probe (position 71 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::reject::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 61 11) (end 61 19)) (probe (position 61 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 59 17) (end 59 25)) (probe (position 59 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::select::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 67 11) (end 67 19)) (probe (position 67 11))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 67 28) (end 67 38)) (probe (position 67 28))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "collection")
      (outcome (status resolved) (target (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection")))))
  )
  (query (document "memory://snapshot/control_functions.md") (range (start 65 17) (end 65 25)) (probe (position 65 17))
    (reference (id (source (node (document "memory://snapshot/control_functions.md") (qualified-name "ControlFunctions::selectOne::collection"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
)
~~~
