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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "02_part_definition_example.md"
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
        (range (start 4 19) (end 4 23))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "944636373dda12837c6ccff46af36fd09e4f675c1adf25758da19456f11c9fde") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Part Definition Example"))) (kind "package") (name "Part Definition Example") (declared-name "Part Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 361))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Part Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 17) (character 1)) (end (line 17) (character 17))) (parent (node (document "d0") (qualified-name "Part Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Person"))) (kind "part def") (name "Person") (declared-name "Person") (range (start (line 18) (character 1)) (end (line 18) (character 17))) (parent (node (document "d0") (qualified-name "Part Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 138))) (parent (node (document "d0") (qualified-name "Part Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind "ref") (name "driver") (declared-name "driver") (range (start (line 9) (character 2)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person") (range (start (line 9) (character 20)) (end (line 9) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 7) (character 2)) (end (line 7) (character 20))) (parent (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 7) (character 13)) (end (line 7) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 4) (character 2)) (end (line 4) (character 24))) (parent (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 4) (character 19)) (end (line 4) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (kind "attribute") (name "status") (declared-name "status") (range (start (line 5) (character 2)) (end (line 5) (character 35))) (parent (node (document "d0") (qualified-name "Part Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStatus") (range none)) (typing (reference "VehicleStatus") (range (start (line 5) (character 21)) (end (line 5) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (kind "attribute def") (name "VehicleStatus") (declared-name "VehicleStatus") (range (start (line 12) (character 1)) (end (line 12) (character 108))) (parent (node (document "d0") (qualified-name "Part Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (kind "attribute") (name "acceleratorPosition") (declared-name "acceleratorPosition") (range (start (line 14) (character 2)) (end (line 14) (character 39))) (parent (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (kind "attribute") (name "gearSetting") (declared-name "gearSetting") (range (start (line 13) (character 2)) (end (line 13) (character 34))) (parent (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range (start (line 9) (character 20)) (end (line 9) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Part Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 7) (character 13)) (end (line 7) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Part Definition Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 4) (character 19)) (end (line 4) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStatus") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus")))))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 1)) (authored-target "VehicleStatus") (range (start (line 5) (character 21)) (end (line 5) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus")))))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::acceleratorPosition"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus::gearSetting"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (target (node (document "d0") (qualified-name "Part Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Part Definition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (target (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (target (node (document "d0") (qualified-name "Part Definition Example::VehicleStatus"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Part Definition Example::Vehicle::status"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
