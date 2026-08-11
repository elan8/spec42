# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_1.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
LineComment,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-1''
    (line_comment)
    (part_def 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'smallVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {

    // Definitions

    part def Vehicle {
        part eng : Engine;
    }

    part def Engine {
        part cyl : Cylinder[4..6];
    }

    part def Cylinder;

    // Usages

    part smallVehicle : Vehicle {
        part redefines eng {
            part redefines cyl[4];
        }
    }

    part bigVehicle : Vehicle {
        part redefines eng {
            part redefines cyl[6];
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5237fc4f7f88fbf13ee074fc04d6e22677877a85a9c824d03e611da74082f2be") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts Example-1"))) (kind "package") (name "Parts Example-1") (declared-name "Parts Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 359))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 12) (character 1)) (end (line 12) (character 19))) (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 8) (character 1)) (end (line 8) (character 50))) (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 9) (character 13)) (end (line 9) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 4) (character 1)) (end (line 4) (character 43))) (parent (node (document "d0") (qualified-name "Parts Example-1"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 5) (character 2)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 5) (character 13)) (end (line 5) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind "part") (name "bigVehicle") (declared-name "bigVehicle") (range (start (line 22) (character 1)) (end (line 22) (character 84))) (parent (node (document "d0") (qualified-name "Parts Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 22) (character 19)) (end (line 22) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind "part") (name "eng") (range (start (line 23) (character 2)) (end (line 23) (character 52))) (parent (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 23) (character 17)) (end (line 23) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 24) (character 3)) (end (line 24) (character 25))) (parent (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 24) (character 18)) (end (line 24) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind "part") (name "smallVehicle") (declared-name "smallVehicle") (range (start (line 16) (character 1)) (end (line 16) (character 86))) (parent (node (document "d0") (qualified-name "Parts Example-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 16) (character 21)) (end (line 16) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind "part") (name "eng") (range (start (line 17) (character 2)) (end (line 17) (character 52))) (parent (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 17) (character 17)) (end (line 17) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 18) (character 3)) (end (line 18) (character 25))) (parent (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 18) (character 18)) (end (line 18) (character 21)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 9) (character 13)) (end (line 9) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 5) (character 13)) (end (line 5) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 22) (character 19)) (end (line 22) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 23) (character 17)) (end (line 23) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 24) (character 18)) (end (line 24) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 16) (character 21)) (end (line 16) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 17) (character 17)) (end (line 17) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 18) (character 18)) (end (line 18) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
