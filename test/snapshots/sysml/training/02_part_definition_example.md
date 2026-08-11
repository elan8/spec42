# META
~~~ini
description=SysML Training 02 (Part Definitions): Part Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Part Definition Example' {
	private import ScalarValues::*;
	
	part def Vehicle {
		attribute mass : Real;
		attribute status : VehicleStatus;
		
		part eng : Engine;
		
		ref part driver : Person;
	}
	
	attribute def VehicleStatus {
		attribute gearSetting : Integer;
		attribute acceleratorPosition : Real;
	}
	
	part def Engine;	
	part def Person;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Part Definition Example''
    (import_decl private 'ScalarValues::*')
    (part_def 'Vehicle'
      (attribute_usage 'mass' : 'Real')
      (attribute_usage 'status' : 'VehicleStatus')
      (part_usage 'eng' : 'Engine')
      (part_usage ref 'driver' : 'Person'))
    (attribute_def 'VehicleStatus'
      (attribute_usage 'gearSetting' : 'Integer')
      (attribute_usage 'acceleratorPosition' : 'Real'))
    (part_def 'Engine')
    (part_def 'Person')))
~~~
# FORMAT
~~~sysml
package 'Part Definition Example' {
    private import ScalarValues::*;

    part def Vehicle {
        attribute mass : Real;
        attribute status : VehicleStatus;

        part eng : Engine;

        ref part driver : Person;
    }

    attribute def VehicleStatus {
        attribute gearSetting : Integer;
        attribute acceleratorPosition : Real;
    }

    part def Engine;
    part def Person;
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Part Definition Example"))) (name "Part Definition Example") (declared-name "Part Definition Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Part Definition Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Part Definition Example::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Part Definition Example::Person"))) (name "Person") (declared-name "Person") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (name "driver") (declared-name "driver") (declared (properties (composite false) (reference true) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "Part Definition Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Part Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Part Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (name "status") (declared-name "status") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Part Definition Example::Vehicle")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (name "VehicleStatus") (declared-name "VehicleStatus") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (name "acceleratorPosition") (declared-name "acceleratorPosition") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (name "gearSetting") (declared-name "gearSetting") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (to (node (document "d0") (qualified-name "Part Definition Example::Person"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Part Definition Example::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (to (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Person"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/02_part_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 2) (end 4 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 39))
      )
    )
  )
)
~~~
