# META
~~~ini
description=SysML Training 35 (Use Cases): Use Case Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Use Case Definition Example' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case def 'Provide Transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Transport driver and passengers from starting location 
			 * to ending location.
			 */
		}		
	}
	
	use case def 'Enter Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case def 'Exit Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/35_use_case_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:22a22d3f68546489437cb3d1d3d78bc12c017b4a16cd8038fb540e8059e3a556") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Fuel Station"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Environment")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Transport driver and passengers from starting location \n\t\t\t * to ending location.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0))
      (authored-target "Environment")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::objective"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::objective")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation")))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle")))
      (featured-by (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation")))
      (type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 24 17) (end 24 23)) (probe (position 24 17))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 25 21) (end 25 27)) (probe (position 25 21))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 23 20) (end 23 27)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 30 17) (end 30 23)) (probe (position 30 17))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 31 21) (end 31 27)) (probe (position 31 21))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 29 20) (end 29 27)) (probe (position 29 20))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 10 17) (end 10 23)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 12 22) (end 12 33)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0) (authored-target "Environment")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Environment")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Person")))))
    )
  )
  (query (document "memory://snapshot/35_use_case_definition_example.md") (range (start 8 20) (end 8 27)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/35_use_case_definition_example.md") (qualified-name "Use Case Definition Example::Vehicle")))))
    )
  )
)
~~~
