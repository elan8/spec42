# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-1
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-1' {
	private import ScalarValues::Real;
	
	enum def TrafficLightColor {
		enum green;
		enum yellow;
		enum red;
	}
	
	part def TrafficLight {
		attribute currentColor : TrafficLightColor;
	}
	
	part def TrafficLightGo specializes TrafficLight {
		attribute redefines currentColor = TrafficLightColor::green;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "06_enumeration_definitions_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6ea693ded4275f5369f0d34c6f0b5e8e7b8907a8ba91204166be4fe0cb116fc4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (kind "package") (name "Enumeration Definitions-1") (declared-name "Enumeration Definitions-1"))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (kind "part def") (name "TrafficLight") (declared-name "TrafficLight") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind "attribute") (name "currentColor") (declared-name "currentColor") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrafficLightColor")) (typing (reference "TrafficLightColor")))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (kind "enum def") (name "TrafficLightColor") (declared-name "TrafficLightColor") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (kind "enumerated value") (name "green") (declared-name "green") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red"))) (kind "enumerated value") (name "red") (declared-name "red") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow"))) (kind "enumerated value") (name "yellow") (declared-name "yellow") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind "part def") (name "TrafficLightGo") (declared-name "TrafficLightGo") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TrafficLight")))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (kind "attribute") (name "currentColor") (declared-name "currentColor") (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "currentColor")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0)) (authored-target "TrafficLightColor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 1)) (authored-target "TrafficLightColor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0)) (authored-target "TrafficLight") (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (kind redefinition) (ordinal 0)) (authored-target "currentColor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 13 37) (end 13 49)) (probe (position 13 37))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))
        (kind specialization) (ordinal 0) (authored-target "TrafficLight")
        (range (start 13 37) (end 13 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight") (range (start 9 1) (end 9 73)))
        )
      )
    )
    (query (range (start 14 22) (end 14 34)) (probe (position 14 22))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))
        (kind redefinition) (ordinal 0) (authored-target "currentColor")
        (range (start 14 22) (end 14 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor") (range (start 14 2) (end 14 62)))
        )
      )
    )
    (query (range (start 10 27) (end 10 44)) (probe (position 10 27))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))
        (kind featureTyping) (ordinal 1) (authored-target "TrafficLightColor")
        (range (start 10 27) (end 10 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor") (range (start 3 1) (end 3 73)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-1::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
