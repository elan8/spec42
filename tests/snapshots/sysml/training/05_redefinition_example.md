# META
~~~ini
description=SysML Training 05 (Redefinition): Redefinition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6aebff5d3d57240d346f01d77adcb1139020b0341de466f8ba9d34e0a85c07b1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BigEngine")) (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SmallEngine")) (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0))
      (authored-target "BigEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0))
      (authored-target "SmallEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (source inherited) (from (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle")))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng")))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle")))
      (type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")) (source direct))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (source inherited) (from (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))
      (type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (source direct))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (scopes any))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (source inherited) (from (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle")))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng")))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle")))
      (type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (source inherited) (from (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")) (source direct))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")) (scopes any))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")))
      (featured-by (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))
      (type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")) (scopes any))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng")) (scopes any feature))
      (subtype (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 18 23) (end 18 29)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 19 17) (end 19 20)) (probe (position 19 17))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "BigEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 8 24) (end 8 31)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 9 16) (end 9 25)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0) (authored-target "BigEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigEngine")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 9 30) (end 9 33)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 13 13) (end 13 21)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Cylinder")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 15 25) (end 15 31)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 16 17) (end 16 20)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (path (named (kind package) (name "Redefinition Example")) (named (kind part-def) (name "SmallEngine")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine::cyl")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 5 26) (end 5 33)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 6 18) (end 6 29)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0) (authored-target "SmallEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallEngine")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 6 40) (end 6 43)) (probe (position 6 40))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng")))))
    )
  )
  (query (document "memory://snapshot/05_redefinition_example.md") (range (start 3 13) (end 3 19)) (probe (position 3 13))
    (reference (id (source (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/05_redefinition_example.md") (qualified-name "Redefinition Example::Engine")))))
    )
  )
)
~~~
