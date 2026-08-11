# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "07_parts_example_2.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-2''
    (line_comment)
    (part_def 'Vehicle')
    (part_def 'Engine')
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'Engine'
        (part_usage 'cyl' : 'Cylinder' multiplicity)))
    (part_usage 'smallVehicle' :> 'vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' :> 'vehicle'
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
package 'Parts Example-2' {

    // Definitions

    part def Vehicle;
    part def Engine;
    part def Cylinder;

    // Usages

    part vehicle : Vehicle {
        part eng : Engine {
            part cyl : Cylinder[4..6];
        }
    }

    part smallVehicle :> vehicle {
        part redefines eng {
            part redefines cyl[4];
        }
    }

    part bigVehicle :> vehicle {
        part redefines eng {
            part redefines cyl[6];
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6fb61cb797210f46e7279e25502702da57fc43fe198992e16f7af201e94d8698") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Parts Example-2"))) (kind "package") (name "Parts Example-2") (declared-name "Parts Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 388))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 6) (character 1)) (end (line 6) (character 19))) (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 4) (character 1)) (end (line 4) (character 18))) (parent (node (document "d0") (qualified-name "Parts Example-2"))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind "part") (name "bigVehicle") (declared-name "bigVehicle") (range (start (line 22) (character 1)) (end (line 22) (character 85))) (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 22) (character 20)) (end (line 22) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind "part") (name "eng") (range (start (line 23) (character 2)) (end (line 23) (character 52))) (parent (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 23) (character 17)) (end (line 23) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 24) (character 3)) (end (line 24) (character 25))) (parent (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 24) (character 18)) (end (line 24) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind "part") (name "smallVehicle") (declared-name "smallVehicle") (range (start (line 16) (character 1)) (end (line 16) (character 87))) (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 16) (character 22)) (end (line 16) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind "part") (name "eng") (range (start (line 17) (character 2)) (end (line 17) (character 52))) (parent (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 17) (character 17)) (end (line 17) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 18) (character 3)) (end (line 18) (character 25))) (parent (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 18) (character 18)) (end (line 18) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 1)) (end (line 10) (character 84))) (parent (node (document "d0") (qualified-name "Parts Example-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 11) (character 2)) (end (line 11) (character 55))) (parent (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 11) (character 13)) (end (line 11) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 12) (character 3)) (end (line 12) (character 29))) (parent (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 12) (character 14)) (end (line 12) (character 22)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 22) (character 20)) (end (line 22) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 23) (character 17)) (end (line 23) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 24) (character 18)) (end (line 24) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 16) (character 22)) (end (line 16) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 17) (character 17)) (end (line 17) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 18) (character 18)) (end (line 18) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 11) (character 13)) (end (line 11) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 12) (character 14)) (end (line 12) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Parts Example-2::Cylinder")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (target (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (target (node (document "d0") (qualified-name "Parts Example-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
