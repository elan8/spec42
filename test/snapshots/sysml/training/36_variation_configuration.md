# META
~~~ini
description=SysML Training 36 (Variability): Variation Configuration
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Configuration' {
	private import 'Variation Usages'::*;
	
	part vehicle4Cyl :> vehicleFamily {
		part redefines engine = engine::'4cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
	part vehicle6Cyl :> vehicleFamily {
		part redefines engine = engine::'6cylEngine';
		part redefines transmission = transmission::manualTransmission;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Configuration''
    (import_decl private ''Variation Usages'::*')
    (part_usage 'vehicle4Cyl' :> 'vehicleFamily'
      (part_usage :>> 'engine' value)
      (part_usage :>> 'transmission' value))
    (part_usage 'vehicle6Cyl' :> 'vehicleFamily'
      (part_usage :>> 'engine' value)
      (part_usage :>> 'transmission' value))))
~~~
# FORMAT
~~~sysml
package 'Variation Configuration' {
    private import 'Variation Usages'::*;

    part vehicle4Cyl :> vehicleFamily {
        part redefines engine = engine::'4cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }

    part vehicle6Cyl :> vehicleFamily {
        part redefines engine = engine::'6cylEngine';
        part redefines transmission = transmission::manualTransmission;
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'vehicleFamily'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'transmission'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Variation Configuration"))) (name "Variation Configuration") (declared-name "Variation Configuration")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Variation Configuration::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (name "vehicle4Cyl") (declared-name "vehicle4Cyl") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (name "engine") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "engine::4cylEngine")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (name "transmission") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "transmission::manualTransmission")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (name "vehicle6Cyl") (declared-name "vehicle6Cyl") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (name "engine") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "engine::6cylEngine")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (name "transmission") (declared (properties (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "transmission::manualTransmission")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle4Cyl::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Variation Configuration::vehicle6Cyl::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/36_variation_configuration.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 4 2) (end 4 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 5 2) (end 5 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 9 2) (end 9 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 10 2) (end 10 65))
      )
    )
  )
)
~~~
