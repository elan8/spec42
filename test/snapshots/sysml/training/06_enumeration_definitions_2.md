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
  (document "06_enumeration_definitions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 38))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "635d883e77284a70cfc0437f177dc3d33fe5b896dca319797f46b95df838b2d0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (kind "package") (name "Enumeration Definitions-2") (declared-name "Enumeration Definitions-2") (range (start (line 0) (character 0)) (end (line 0) (character 633))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 47))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Enumeration Definitions-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind "enum def") (name "ClassificationKind") (declared-name "ClassificationKind") (range (start (line 9) (character 1)) (end (line 9) (character 310))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ClassificationLevel") (range (start (line 9) (character 41)) (end (line 9) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::confidential"))) (kind "enumerated value") (name "confidential") (declared-name "confidential") (range (start (line 14) (character 2)) (end (line 14) (character 14))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::secret"))) (kind "enumerated value") (name "secret") (declared-name "secret") (range (start (line 18) (character 2)) (end (line 18) (character 8))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind::unclassified"))) (kind "enumerated value") (name "unclassified") (declared-name "unclassified") (range (start (line 10) (character 2)) (end (line 10) (character 14))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (kind "attribute def") (name "ClassificationLevel") (declared-name "ClassificationLevel") (range (start (line 4) (character 1)) (end (line 4) (character 105))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (kind "attribute") (name "code") (declared-name "code") (range (start (line 5) (character 2)) (end (line 5) (character 26))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (kind "attribute") (name "color") (declared-name "color") (range (start (line 6) (character 2)) (end (line 6) (character 38))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrafficLightColor") (range none)))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))) (kind "enum def") (name "GradePoints") (declared-name "GradePoints") (range (start (line 24) (character 1)) (end (line 24) (character 89))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Real") (range (start (line 24) (character 25)) (end (line 24) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::A"))) (kind "enumerated value") (name "A") (declared-name "A") (range (start (line 25) (character 2)) (end (line 25) (character 3))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::B"))) (kind "enumerated value") (name "B") (declared-name "B") (range (start (line 26) (character 2)) (end (line 26) (character 3))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::C"))) (kind "enumerated value") (name "C") (declared-name "C") (range (start (line 27) (character 2)) (end (line 27) (character 3))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::D"))) (kind "enumerated value") (name "D") (declared-name "D") (range (start (line 28) (character 2)) (end (line 28) (character 3))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))))
    (element (id (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints::F"))) (kind "enumerated value") (name "F") (declared-name "F") (range (start (line 29) (character 2)) (end (line 29) (character 3))) (parent (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Enumeration Definitions-1::*") (range (start (line 2) (character 16)) (end (line 2) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind specialization) (ordinal 0)) (authored-target "ClassificationLevel") (range (start (line 9) (character 41)) (end (line 9) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::code"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel::color"))) (kind featureTyping) (ordinal 0)) (authored-target "TrafficLightColor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))) (kind specialization) (ordinal 0)) (authored-target "Real") (range (start (line 24) (character 25)) (end (line 24) (character 29))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (target (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 24 25) (end 24 29)) (probe (position 24 25))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-2::GradePoints"))
        (kind specialization) (ordinal 0) (authored-target "Real")
        (range (start 24 25) (end 24 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 41) (end 9 60)) (probe (position 9 41))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationKind"))
        (kind specialization) (ordinal 0) (authored-target "ClassificationLevel")
        (range (start 9 41) (end 9 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Enumeration Definitions-2::ClassificationLevel") (range (start 4 1) (end 4 105)))
        )
      )
    )
    (query (range (start 2 16) (end 2 43)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Enumeration Definitions-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Enumeration Definitions-1::*")
        (range (start 2 16) (end 2 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
