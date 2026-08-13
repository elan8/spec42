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
  (document "memory://snapshot/measurement_ref_calculations.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 29) (end 13 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 13 50) (end 13 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 13 76) (end 13 101))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 13 102) (end 13 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 29) (end 14 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 50) (end 14 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 76) (end 14 101))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 102) (end 14 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 30) (end 15 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 52) (end 15 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 78) (end 15 92))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 93) (end 15 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 29) (end 16 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 50) (end 16 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 76) (end 16 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 91) (end 16 119))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 44) (end 19 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 65) (end 19 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 91) (end 19 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 117) (end 19 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 44) (end 20 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 20 65) (end 20 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 20 91) (end 20 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 20 117) (end 20 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 34) (end 22 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 27 8) (end 27 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 27 45) (end 27 64))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:cb3bb9a6969b967cd69ea4dd046c1ecf38b7addd6d6c11b4a2ac44318655f72b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::MeasurementUnit") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::ScalarMeasurementReference") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::CoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::*"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::**"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::**"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::/"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::/"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::/"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::ToString"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::ToString"))))
    (declaration (id (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::^"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::^"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::MeasurementUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::ScalarMeasurementReference")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::*"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::**"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::**")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::/"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::ToString"))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::^"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::^")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 6 19) (end 6 39)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 7 19) (end 7 37)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 8 19) (end 8 57)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::MeasurementUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 9 19) (end 9 68)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ScalarMeasurementReference")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 10 19) (end 10 57)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 13 29) (end 13 47)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::*"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 15 30) (end 15 49)) (probe (position 15 30))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::**"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::**")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 14 29) (end 14 47)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::/"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 19 44) (end 19 62)) (probe (position 19 44))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame*"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 20 44) (end 20 62)) (probe (position 20 44))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::CoordinateFrame/"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 22 34) (end 22 57)) (probe (position 22 34))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::ToString"))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::ToString")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_ref_calculations.md") (range (start 16 29) (end 16 47)) (probe (position 16 29))
    (reference (id (source (node (document "memory://snapshot/measurement_ref_calculations.md") (qualified-name "MeasurementRefCalculations::^"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::^")
      (outcome (status unresolved)))
  )
)
~~~
