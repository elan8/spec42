# META
~~~ini
description=SysML Training 04 (Subsetting): Subsetting Example
type=file
~~~
# SOURCE
~~~sysml
package 'Subsetting Example' {
	
	part def Vehicle {
		part parts : VehiclePart[*];
		
		part eng : Engine subsets parts;
		part trans : Transmission subsets parts;
		part wheels : Wheel[4] :> parts;
	}
	
	abstract part def VehiclePart;
	part def Engine :> VehiclePart;
	part def Transmission :> VehiclePart;
	part def Wheel :> VehiclePart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/04_subsetting_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:41d012599cbb78a05efcfb16c6d706d6c0b41992e72679d6e214d594a790f12d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehiclePart")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehiclePart")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")) (subsetting (reference "parts")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehiclePart")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")) (subsetting (reference "parts")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")) (subsetting (reference "parts")))))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehiclePart")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (kind specialization) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (kind specialization) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (kind specialization) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng")))
      (featured-by (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle")))
      (type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")) (source direct))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (source inherited) (from (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))
      (featured-by (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle")))
      (type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (source direct))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng")) (scopes any feature))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans")) (scopes any feature))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans")))
      (featured-by (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle")))
      (type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")) (source direct))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (source inherited) (from (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")) (scopes any))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels")))
      (featured-by (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle")))
      (type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (source inherited) (from (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))))
      (effective-type (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")) (scopes any))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")))
      (supertype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 11 20) (end 11 31)) (probe (position 11 20))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine"))) (kind specialization) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 12 26) (end 12 37)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission"))) (kind specialization) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 5 13) (end 5 19)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Engine")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::eng"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 3 15) (end 3 26)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts"))) (kind featureTyping) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 6 15) (end 6 27)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Transmission")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 6 36) (end 6 41)) (probe (position 6 36))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::trans"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 7 28) (end 7 33)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::wheels"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Vehicle::parts")))))
    )
  )
  (query (document "memory://snapshot/04_subsetting_example.md") (range (start 13 19) (end 13 30)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::Wheel"))) (kind specialization) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/04_subsetting_example.md") (qualified-name "Subsetting Example::VehiclePart")))))
    )
  )
)
~~~
