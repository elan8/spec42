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
  (document "control_functions.md"
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
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ae3684db479e2de911a2d35d859a5e04d1d20a6151c6b9832402070ec2a2e2f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ControlFunctions"))) (kind "package") (name "ControlFunctions") (declared-name "ControlFunctions"))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::ScalarValue"))) (kind "import") (name "ScalarValue") (declared-name "ScalarValue") (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::ScalarValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::allTrue"))) (kind "kermlDecl") (name "allTrue") (declared-name "allTrue") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::anyTrue"))) (kind "kermlDecl") (name "anyTrue") (declared-name "anyTrue") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::collect"))) (kind "kermlDecl") (name "collect") (declared-name "collect") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::exists"))) (kind "kermlDecl") (name "exists") (declared-name "exists") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::forAll"))) (kind "kermlDecl") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::in"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::in#kermlDecl"))) (kind "kermlDecl") (name "in") (declared-name "in") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::max"))) (kind "import") (name "max") (declared-name "max") (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::max") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::maximize"))) (kind "kermlDecl") (name "maximize") (declared-name "maximize") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::min"))) (kind "import") (name "min") (declared-name "min") (parent (node (document "d0") (qualified-name "ControlFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::min") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::minimize"))) (kind "kermlDecl") (name "minimize") (declared-name "minimize") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::reduce"))) (kind "kermlDecl") (name "reduce") (declared-name "reduce") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::reject"))) (kind "kermlDecl") (name "reject") (declared-name "reject") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::select"))) (kind "kermlDecl") (name "select") (declared-name "select") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
    (element (id (node (document "d0") (qualified-name "ControlFunctions::selectOne"))) (kind "kermlDecl") (name "selectOne") (declared-name "selectOne") (parent (node (document "d0") (qualified-name "ControlFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::ScalarValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::ScalarValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::max"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarFunctions::max") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ControlFunctions::min"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarFunctions::min") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ControlFunctions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 36)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "ControlFunctions::min"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarFunctions::min")
        (range (start 10 16) (end 10 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 36)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "ControlFunctions::max"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarFunctions::max")
        (range (start 11 16) (end 11 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 37)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "ControlFunctions::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 9 16) (end 9 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 41)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ControlFunctions::ScalarValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::ScalarValue")
        (range (start 8 16) (end 8 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
