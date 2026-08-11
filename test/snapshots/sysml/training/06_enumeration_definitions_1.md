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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6ea693ded4275f5369f0d34c6f0b5e8e7b8907a8ba91204166be4fe0cb116fc4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (kind "package") (name "Enumeration Definitions-1") (declared-name "Enumeration Definitions-1") (range (start (line 0) (character 0)) (end (line 0) (character 347))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (kind "part def") (name "TrafficLight") (declared-name "TrafficLight") (range (start (line 9) (character 1)) (end (line 9) (character 73))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind "attribute") (name "currentColor") (declared-name "currentColor") (range (start (line 10) (character 2)) (end (line 10) (character 45))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrafficLightColor") (range none)) (typing (reference "TrafficLightColor") (range (start (line 10) (character 27)) (end (line 10) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (kind "enum def") (name "TrafficLightColor") (declared-name "TrafficLightColor") (range (start (line 3) (character 1)) (end (line 3) (character 73))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (kind "enumerated value") (name "green") (declared-name "green") (range (start (line 4) (character 7)) (end (line 4) (character 12))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red"))) (kind "enumerated value") (name "red") (declared-name "red") (range (start (line 6) (character 7)) (end (line 6) (character 10))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow"))) (kind "enumerated value") (name "yellow") (declared-name "yellow") (range (start (line 5) (character 7)) (end (line 5) (character 13))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind "part def") (name "TrafficLightGo") (declared-name "TrafficLightGo") (range (start (line 13) (character 1)) (end (line 13) (character 117))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TrafficLight") (range (start (line 13) (character 37)) (end (line 13) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (kind "attribute") (name "currentColor") (declared-name "currentColor") (range (start (line 14) (character 2)) (end (line 14) (character 62))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "currentColor") (range (start (line 14) (character 22)) (end (line 14) (character 34)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0)) (authored-target "TrafficLightColor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 1)) (authored-target "TrafficLightColor") (range (start (line 10) (character 27)) (end (line 10) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0)) (authored-target "TrafficLight") (range (start (line 13) (character 37)) (end (line 13) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLight")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor"))) (kind redefinition) (ordinal 0)) (authored-target "currentColor") (range (start (line 14) (character 22)) (end (line 14) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-1::TrafficLightGo::currentColor")))))
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
