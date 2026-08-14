# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-2' {
	private import ScalarValues::*;
	private import 'Enumeration Definitions-1'::*;
	
	attribute def ClassificationLevel {
		attribute code : String;
		attribute color : TrafficLightColor;
	}
	
	enum def ClassificationKind specializes ClassificationLevel {
		unclassified {
			:>> code = "uncl";
			:>> color = TrafficLightColor::green;
		}
		confidential {
			:>> code = "conf";
			:>> color = TrafficLightColor::yellow;
		}
		secret {
			:>> code = "secr";
			:>> color = TrafficLightColor::red;
		}
	}
	
	enum def GradePoints :> Real {
		A = 4.0;
		B = 3.0;
		C = 2.0;
		D = 1.0;
		F = 0.0;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/06_enumeration_definitions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 19) (end 5 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 20) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 24 25) (end 24 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d0145d2f5251b644319fef2f379abcca21d2c4061201f2a805154b28f0b64957") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Enumeration Definitions-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind::confidential"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind::secret"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind::unclassified"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrafficLightColor"))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints::A"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints::B"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints::C"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints::D"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints::F"))) (kind enum-literal) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Enumeration Definitions-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind specialization) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrafficLightColor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (target (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 2 16) (end 2 46)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (path (named (kind package) (name "Enumeration Definitions-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "Enumeration Definitions-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 9 41) (end 9 60)) (probe (position 9 41))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind specialization) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 5 19) (end 5 25)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 6 20) (end 6 37)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (kind featureTyping) (ordinal 0) (authored-target "TrafficLightColor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/06_enumeration_definitions_2.md") (range (start 24 25) (end 24 29)) (probe (position 24 25))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_2.md") (qualified-name "Enumeration Definitions-2::GradePoints"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
