# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/MeasurementRefCalculations
type=file
~~~
# SOURCE
~~~sysml
standard library package MeasurementRefCalculations {
	doc
	/*
	 * This package package defines calculations on MeasurementUnits and CoordinateFrames.
	 */
	 
    private import ScalarValues::String;
    private import ScalarValues::Real;
    private import MeasurementReferences::MeasurementUnit;
    private import MeasurementReferences::ScalarMeasurementReference;
    private import MeasurementReferences::CoordinateFrame;
        
    /* MeasurementUnit operations */
    calc def '*' specializes DataFunctions::'*' { in x: MeasurementUnit[1]; in y: MeasurementUnit[1]; return : MeasurementUnit[1]; }
    calc def '/' specializes DataFunctions::'/' { in x: MeasurementUnit[1]; in y: MeasurementUnit[1]; return : MeasurementUnit[1]; }
    calc def '**' specializes DataFunctions::'**' { in x: MeasurementUnit[1]; in y: Real[1]; return : MeasurementUnit[1]; }
    calc def '^' specializes DataFunctions::'^' { in x: MeasurementUnit[1]; in y: Real[1]; return : MeasurementUnit[1]; }

    /* CoordinateFrame and MeasurementUnit operations */
    calc def 'CoordinateFrame*' specializes DataFunctions::'*' { in x: CoordinateFrame[1]; in y: MeasurementUnit[1]; return : CoordinateFrame[1]; }
    calc def 'CoordinateFrame/' specializes DataFunctions::'/' { in x: CoordinateFrame[1]; in y: MeasurementUnit[1]; return : CoordinateFrame[1]; }

    calc def ToString specializes BaseFunctions::ToString { 
        doc 
        /*
         * Returns the Unicode string symbol representing a scalar measurement reference.
         */
        in x: ScalarMeasurementReference[1]; return : String[1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "measurement_ref_calculations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 50) (end 13 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 76) (end 13 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 50) (end 14 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 76) (end 14 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 52) (end 15 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 78) (end 15 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 50) (end 16 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 76) (end 16 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 65) (end 19 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 91) (end 19 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 65) (end 20 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 91) (end 20 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 8) (end 27 44))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1501347aec9c648148b7c97ef5a4211165d7b1c31fadaca4b0f7441513ee555c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (kind "package") (name "MeasurementRefCalculations") (declared-name "MeasurementRefCalculations"))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*"))) (kind "calc def") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**"))) (kind "calc def") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::**"))) (authored (relationships (typing (reference "x: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::**::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::**"))) (authored (relationships (typing (reference "y: Real[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::*"))) (authored (relationships (typing (reference "x: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::*::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::*"))) (authored (relationships (typing (reference "y: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/"))) (kind "calc def") (name "/") (declared-name "/") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::/"))) (authored (relationships (typing (reference "x: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::/::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::/"))) (authored (relationships (typing (reference "y: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (kind "calc def") (name "CoordinateFrame*") (declared-name "CoordinateFrame*") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (authored (relationships (typing (reference "x: CoordinateFrame[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (authored (relationships (typing (reference "y: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (kind "calc def") (name "CoordinateFrame/") (declared-name "CoordinateFrame/") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (authored (relationships (typing (reference "x: CoordinateFrame[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (authored (relationships (typing (reference "y: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::MeasurementUnit"))) (kind "import") (name "MeasurementUnit") (declared-name "MeasurementUnit") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::MeasurementUnit") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ScalarMeasurementReference"))) (kind "import") (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::ScalarMeasurementReference") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))) (kind "calc def") (name "ToString") (declared-name "ToString") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString"))) (authored (relationships (typing (reference "x: ScalarMeasurementReference[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^"))) (kind "calc def") (name "^") (declared-name "^") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^::x"))) (kind "in out parameter") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::^"))) (authored (relationships (typing (reference "x: MeasurementUnit[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::^::y"))) (kind "in out parameter") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations::^"))) (authored (relationships (typing (reference "y: Real[1]")))))
    (element (id (node (document "d0") (qualified-name "MeasurementRefCalculations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementRefCalculations"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::**::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::**::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: Real[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::*::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::*::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::/::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::/::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: CoordinateFrame[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame*::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: CoordinateFrame[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame/::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::MeasurementUnit"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::MeasurementUnit") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::ScalarMeasurementReference"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::ScalarMeasurementReference") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::ToString::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: ScalarMeasurementReference[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::^::x"))) (kind featureTyping) (ordinal 0)) (authored-target "x: MeasurementUnit[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementRefCalculations::^::y"))) (kind featureTyping) (ordinal 0)) (authored-target "y: Real[1]") (outcome (status unresolved)))
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
    (query (range (start 7 19) (end 7 37)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementRefCalculations::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 7 19) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 39)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementRefCalculations::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 6 19) (end 6 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 57)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementRefCalculations::MeasurementUnit"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::MeasurementUnit")
        (range (start 8 19) (end 8 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 57)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementRefCalculations::CoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
        (range (start 10 19) (end 10 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 68)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementRefCalculations::ScalarMeasurementReference"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ScalarMeasurementReference")
        (range (start 9 19) (end 9 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
