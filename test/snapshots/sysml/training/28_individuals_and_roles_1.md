# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Roles-1
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Roles' {
	private import 'Part Definition Example'::*;
	
	part def Wheel;
	
	individual part def Vehicle_1 :> Vehicle {
		part leftFrontWheel : Wheel;
		part rightFrontWheel : Wheel;
	}
	
	individual part def Wheel_1 :> Wheel;
	
	individual part vehicle_1 : Vehicle_1 {
		snapshot part vehicle_1_t0 {
			snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
		}
		
		then snapshot part vehicle_1_t1 {
			snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_roles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 34) (end 5 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 24) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 25) (end 7 30))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 2) (end 13 201))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwPart,Ident,Colon,Ident,OpenCurly,
KwSnapshot,KwPart,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwThen,KwSnapshot,KwPart,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Individuals and Roles''
    (import_decl private ''Part Definition Example'::*')
    (part_def 'Wheel')
    (part_def individual 'Vehicle_1' :> 'Vehicle'
      (part_usage 'leftFrontWheel' : 'Wheel')
      (part_usage 'rightFrontWheel' : 'Wheel'))
    (part_def individual 'Wheel_1' :> 'Wheel')
    (part_usage individual 'vehicle_1' : 'Vehicle_1'
      (malformed)
      (part_usage 'vehicle_1_t0'
        (portion_usage snapshot 'leftFrontWheel_t0' : 'Wheel_1' :>> 'leftFrontWheel'))
      (source_succession
        (malformed))
      (part_usage 'vehicle_1_t1'
        (portion_usage snapshot 'rightFrontWheel_t1' : 'Wheel_1' :>> 'rightFrontWheel')))))
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.unresolved_name 'Vehicle'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.unresolved_name 'Vehicle'
~~~
# FORMAT
~~~sysml
package 'Individuals and Roles' {
    private import 'Part Definition Example'::*;

    part def Wheel;

    individual part def Vehicle_1 :> Vehicle {
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    individual part def Wheel_1 :> Wheel;

    individual part vehicle_1 : Vehicle_1 {
        snapshot part vehicle_1_t0 {
            snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
        }

        then snapshot part vehicle_1_t1 {
            snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ddc7839d525f5f08f9247427e5750b72acf146740fb7a4bb7e0b3988f089a4db") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Roles"))) (kind "package") (name "Individuals and Roles") (declared-name "Individuals and Roles") (range (start (line 0) (character 0)) (end (line 0) (character 499))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Import) (visibility "private") (import (reference "Part Definition Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (range (start (line 5) (character 1)) (end (line 5) (character 109))) (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 5) (character 34)) (end (line 5) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (range (start (line 6) (character 2)) (end (line 6) (character 30))) (parent (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 6) (character 24)) (end (line 6) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (range (start (line 7) (character 2)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 7) (character 25)) (end (line 7) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 3) (character 1)) (end (line 3) (character 16))) (parent (node (document "d0") (qualified-name "Individuals and Roles"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind "part def") (name "Wheel_1") (declared-name "Wheel_1") (range (start (line 10) (character 1)) (end (line 10) (character 38))) (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel") (range (start (line 10) (character 32)) (end (line 10) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind "part") (name "vehicle_1") (declared-name "vehicle_1") (range (start (line 12) (character 1)) (end (line 12) (character 243))) (parent (node (document "d0") (qualified-name "Individuals and Roles"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle_1") (range (start (line 12) (character 29)) (end (line 12) (character 38)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Part Definition Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 5) (character 34)) (end (line 5) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 6) (character 24)) (end (line 6) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 7) (character 25)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (range (start (line 10) (character 32)) (end (line 10) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Roles::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle_1") (range (start (line 12) (character 29)) (end (line 12) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (target (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (target (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
