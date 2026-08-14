# META
~~~ini
description=KerML Simple Tests: Features
type=file
~~~
# SOURCE
~~~kerml
package Features {
	classifier A;
	classifier B;
	
	feature f;
	feature g;
	
	feature x typed by A, B references f subsets g;
	
	// Equivalent declaration:
	feature x1 subsets g typed by A subsets f typed by B;
	
	classifier C;
	
	feature y;
	featuring F of y by C;
	
	feature y1 : A :> x featured by C;
	
	feature z unions f, g disjoint from y;
	feature z1 intersects f,g differences y, y1, z;
	
	classifier Person;
	
	abstract feature person : Person; // Default subsets Base::things.
	feature child subsets person;
	
	feature adult differences person, child;
	
	classifier Fuel;
	
	classifier Tanks {
	    feature fuelInPort {
	        in feature fuelFlow : Fuel;
	    }
	    feature fuelOutPort ~ fuelInPort;
	}
	
	feature parent[1..2] : Person;
	feature mother : Person[1] :> parent;
	
	specialization t1 typing f typed by B;
	specialization t2 typing g : A;
	
	specialization Sub subset parent subsets person;
	specialization subset mother subsets parent;
	
	classifier LegalRecord {
		feature guardian[1];
	}
	
    class RegisteredAsset {
        composite var feature identifier[0..1];
    }
    
    classifier Vehicle :> RegisteredAsset {
        derived var feature vin[1] = identifier;
        
        var feature v : Vehicle;
        binding vin = v.vin;
        var feature w = v.vin;
        
        feature x = vin;
        binding x = vin;
    }
	feature legalIdentification;
	
	specialization Redef redefinition LegalRecord::guardian redefines parent;
	specialization redefinition Vehicle::vin redefines RegisteredAsset::identifier;
	
	redefinition Vehicle::vin redefines legalIdentification; 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 1) (end 1 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 20) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 23) (end 7 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 1) (end 10 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 1) (end 12 14))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 15 1) (end 17 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 17 1) (end 17 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 1) (end 17 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 20 1) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 1) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 22 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 27) (end 24 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 27 1) (end 27 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 1) (end 27 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 31) (end 33 35))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 35 5) (end 36 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 24) (end 38 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 18) (end 39 24))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 1) (end 51 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 52 8) (end 53 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 37) (end 56 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 22) (end 59 27))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 67 1) (end 71 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:00b61c0e836d1cea4e3d75c9e05025a79f812727ef20f86d6a231fe523e3fdcd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort::fuelFlow"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel") (direction in)))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RegisteredAsset")))))
    (declaration (id (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "vin")) (bindTarget (reference "v::vin")))))
    (declaration (id (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "x")) (bindTarget (reference "vin")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers derived var) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "identifier")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "v::vin")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "vin")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::child"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "person")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::f"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::g"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::legalIdentification"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")) (subsetting (reference "parent")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::parent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::person"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")) (featureTyping (reference "B")) (subsetting (reference "g")))))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::y"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort::fuelFlow"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "RegisteredAsset")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindSource) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "v::vin")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (kind expressionOperand) (ordinal 0))
      (authored-target "identifier")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "v::vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (kind expressionOperand) (ordinal 0))
      (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::child"))) (kind subsetting) (ordinal 0))
      (authored-target "person")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::person")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind subsetting) (ordinal 0))
      (authored-target "parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::parent")))))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::parent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::person"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind featureTyping) (ordinal 1))
      (authored-target "B")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind subsetting) (ordinal 0))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::g")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::child"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::child"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset")))
      (subtype (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort::fuelFlow")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
      (type (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset")) (scopes any))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x")))
      (featured-by (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::child")))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::person")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::g")))
      (subtype (node (document "memory://snapshot/features.md") (qualified-name "Features::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::mother")))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::parent")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::parent")))
      (subtype (node (document "memory://snapshot/features.md") (qualified-name "Features::mother")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::person")))
      (subtype (node (document "memory://snapshot/features.md") (qualified-name "Features::child")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features::x")))
      (supertype (node (document "memory://snapshot/features.md") (qualified-name "Features::g")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/features.md") (range (start 33 31) (end 33 35)) (probe (position 33 31))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Tanks::fuelInPort::fuelFlow"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 55 26) (end 55 41)) (probe (position 55 26))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "RegisteredAsset")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::RegisteredAsset")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 59 16) (end 59 19)) (probe (position 59 16))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 63 16) (end 63 17)) (probe (position 63 16))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindSource) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 59 22) (end 59 27)) (probe (position 59 22))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "v::vin")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 63 20) (end 63 23)) (probe (position 63 20))
    (reference (id (source (node (document "memory://snapshot/features.md") (path (named (kind package) (name "Features")) (named (kind kerml-classifier) (name "Vehicle")) (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 58 24) (end 58 31)) (probe (position 58 24))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::v"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 56 37) (end 56 47)) (probe (position 56 37))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin"))) (kind expressionOperand) (ordinal 0) (authored-target "identifier")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 60 24) (end 60 29)) (probe (position 60 24))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::w"))) (kind memberAccessOperand) (ordinal 0) (authored-target "v::vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 62 20) (end 62 23)) (probe (position 62 20))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::x"))) (kind expressionOperand) (ordinal 0) (authored-target "vin")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::Vehicle::vin")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 25 23) (end 25 29)) (probe (position 25 23))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::child"))) (kind subsetting) (ordinal 0) (authored-target "person")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::person")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 39 18) (end 39 24)) (probe (position 39 18))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 39 31) (end 39 37)) (probe (position 39 31))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::mother"))) (kind subsetting) (ordinal 0) (authored-target "parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::parent")))))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 38 24) (end 38 30)) (probe (position 38 24))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::parent"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 24 27) (end 24 33)) (probe (position 24 27))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::person"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 7 20) (end 7 21)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 7 23) (end 7 24)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind featureTyping) (ordinal 1) (authored-target "B")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/features.md") (range (start 7 46) (end 7 47)) (probe (position 7 46))
    (reference (id (source (node (document "memory://snapshot/features.md") (qualified-name "Features::x"))) (kind subsetting) (ordinal 0) (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/features.md") (qualified-name "Features::g")))))
    )
  )
)
~~~
