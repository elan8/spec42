# META
~~~ini
description=SysML Training 05 (Redefinition): Redefinition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Redefinition Example''
    (part_def 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'SmallVehicle' :> 'Vehicle'
      (part_usage 'smallEng' : 'SmallEngine' :>> 'eng'))
    (part_def 'BigVehicle' :> 'Vehicle'
      (part_usage 'bigEng' : 'BigEngine' :>> 'eng'))
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'SmallEngine' :> 'Engine'
      (part_usage :>> 'cyl' multiplicity))
    (part_def 'BigEngine' :> 'Engine'
      (part_usage :>> 'cyl' multiplicity))
    (part_def 'Cylinder')))
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
package 'Redefinition Example' {

    part def Vehicle {
        part eng : Engine;
    }
    part def SmallVehicle :> Vehicle {
        part smallEng : SmallEngine redefines eng;
    }
    part def BigVehicle :> Vehicle {
        part bigEng : BigEngine :>> eng;
    }

    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def SmallEngine :> Engine {
        part redefines cyl[4];
    }
    part def BigEngine :> Engine {
        part redefines cyl[6];
    }

    part def Cylinder;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f009f7be8d9cceb567033c041b9ecf87d19176cecfddc9f71e9b662dcdc886e6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Redefinition Example"))) (kind "package") (name "Redefinition Example") (declared-name "Redefinition Example") (range (start (line 0) (character 0)) (end (line 0) (character 430))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind "part def") (name "BigEngine") (declared-name "BigEngine") (range (start (line 18) (character 1)) (end (line 18) (character 59))) (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 18) (character 23)) (end (line 18) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind "part") (name "cyl") (range (start (line 19) (character 2)) (end (line 19) (character 24))) (parent (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 19) (character 17)) (end (line 19) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind "part def") (name "BigVehicle") (declared-name "BigVehicle") (range (start (line 8) (character 1)) (end (line 8) (character 71))) (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 8) (character 24)) (end (line 8) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind "part") (name "bigEng") (declared-name "bigEng") (range (start (line 9) (character 2)) (end (line 9) (character 34))) (parent (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "BigEngine") (range (start (line 9) (character 16)) (end (line 9) (character 25)))) (redefinition (reference "eng") (range (start (line 9) (character 30)) (end (line 9) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 22) (character 1)) (end (line 22) (character 19))) (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 12) (character 1)) (end (line 12) (character 50))) (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 13) (character 2)) (end (line 13) (character 28))) (parent (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 13) (character 13)) (end (line 13) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind "part def") (name "SmallEngine") (declared-name "SmallEngine") (range (start (line 15) (character 1)) (end (line 15) (character 61))) (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 15) (character 25)) (end (line 15) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind "part") (name "cyl") (range (start (line 16) (character 2)) (end (line 16) (character 24))) (parent (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 16) (character 17)) (end (line 16) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind "part def") (name "SmallVehicle") (declared-name "SmallVehicle") (range (start (line 5) (character 1)) (end (line 5) (character 83))) (parent (node (document "d0") (qualified-name "Redefinition Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 5) (character 26)) (end (line 5) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind "part") (name "smallEng") (declared-name "smallEng") (range (start (line 6) (character 2)) (end (line 6) (character 44))) (parent (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "SmallEngine") (range (start (line 6) (character 18)) (end (line 6) (character 29)))) (redefinition (reference "eng") (range (start (line 6) (character 40)) (end (line 6) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 43))) (parent (node (document "d0") (qualified-name "Redefinition Example"))))
    (element (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 3) (character 2)) (end (line 3) (character 20))) (parent (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 3) (character 13)) (end (line 3) (character 19)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 18) (character 23)) (end (line 18) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 19) (character 17)) (end (line 19) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 8) (character 24)) (end (line 8) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0)) (authored-target "BigEngine") (range (start (line 9) (character 16)) (end (line 9) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 9) (character 30)) (end (line 9) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 13) (character 13)) (end (line 13) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 15) (character 25)) (end (line 15) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 16) (character 17)) (end (line 16) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 5) (character 26)) (end (line 5) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0)) (authored-target "SmallEngine") (range (start (line 6) (character 18)) (end (line 6) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 6) (character 40)) (end (line 6) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 3) (character 13)) (end (line 3) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
