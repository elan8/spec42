# META
~~~ini
description=SysML Training 03 (Generalization): Generalization Example
type=file
~~~
# SOURCE
~~~sysml
package 'Generalization Example' {

	abstract part def Vehicle;
	
	part def HumanDrivenVehicle specializes Vehicle {
		ref part driver : Person;
	}
	
	part def PoweredVehicle :> Vehicle {
		part eng : Engine;
	}
	
	part def HumanDrivenPoweredVehicle :> 
		HumanDrivenVehicle, PoweredVehicle;
	
	part def Engine;	
	part def Person;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/03_generalization_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e8d3d3407b2a23c80a57ae811f59b0d24cddac100a30d073f039b56baead36af") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HumanDrivenVehicle")) (specialization (reference "PoweredVehicle")))))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "HumanDrivenVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")))))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1))
      (authored-target "PoweredVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")))))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")))))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle")))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver")))
      (featured-by (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")))
      (type (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")) (source direct))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng")))
      (featured-by (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")))
      (type (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 13 2) (end 13 20)) (probe (position 13 2))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 0) (authored-target "HumanDrivenVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle")))))
    )
  )
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 13 22) (end 13 36)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (kind specialization) (ordinal 1) (authored-target "PoweredVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle")))))
    )
  )
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 4 41) (end 4 48)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 5 20) (end 5 26)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Person")))))
    )
  )
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 8 28) (end 8 35)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/03_generalization_example.md") (range (start 9 13) (end 9 19)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/03_generalization_example.md") (qualified-name "Generalization Example::Engine")))))
    )
  )
)
~~~
