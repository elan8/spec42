# META
~~~ini
description=SysML Training 31 (Constraints): Constraint Assertions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
	}
	
	constraint massConstraint : MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert massConstraint {
			in partMasses = (chassisMass, engine.mass, transmission.mass);
			in massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_constraint_assertions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 13 1) (end 13 146))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 26) (end 26 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 3) (end 29 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 20) (end 29 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 3) (end 33 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 20) (end 33 29))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,CloseParen,LtEq,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,Ident,OpenCurly,
KwIn,Ident,Eq,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Constraint Assertions-2''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'Engine')
    (part_def 'Transmission')
    (constraint_def 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue'))
    (constraint_usage 'massConstraint' : 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue')
      (result_expr_member))
    (part_def 'Vehicle'
      (sysml_decl 'massConstraint'
        (default_ref_usage in 'partMasses' value)
        (default_ref_usage in 'massLimit' value))
      (attribute_usage 'chassisMass' : 'MassValue')
      (part_usage 'engine' : 'Engine'
        (attribute_usage 'mass' : 'MassValue'))
      (part_usage 'transmission' : 'Engine'
        (attribute_usage 'mass' : 'MassValue')))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# FORMAT
~~~sysml
package 'Constraint Assertions-2' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;

    part def Engine;
    part def Transmission;

    constraint def MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;
    }

    constraint massConstraint : MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;

        sum(partMasses) <= massLimit
    }

    part def Vehicle {
        assert massConstraint {
            in partMasses = (chassisMass, engine.mass, transmission.mass);
            in massLimit = 2500[kg];
        }

        attribute chassisMass : MassValue;

        part engine : Engine {
            attribute mass : MassValue;
        }

        part transmission : Engine {
            attribute mass : MassValue;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ab0db59faa4337f266a6fec8fdd401c1798b03ecf9bfce5f4a8aba4491bccd49") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2"))) (kind "package") (name "Constraint Assertions-2") (declared-name "Constraint Assertions-2") (range (start (line 0) (character 0)) (end (line 0) (character 739))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 38))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))) (kind "constraint def") (name "MassConstraint") (declared-name "MassConstraint") (range (start (line 8) (character 1)) (end (line 8) (character 98))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 6) (character 1)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 20) (character 1)) (end (line 20) (character 318))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind "attribute") (name "chassisMass") (declared-name "chassisMass") (range (start (line 26) (character 2)) (end (line 26) (character 36))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 26) (character 26)) (end (line 26) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 28) (character 2)) (end (line 28) (character 59))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 28) (character 16)) (end (line 28) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 29) (character 3)) (end (line 29) (character 30))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 29) (character 20)) (end (line 29) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 32) (character 2)) (end (line 32) (character 65))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 32) (character 22)) (end (line 32) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 33) (character 3)) (end (line 33) (character 30))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 33) (character 20)) (end (line 33) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind "constraint") (name "massConstraint") (declared-name "massConstraint") (range (start (line 13) (character 1)) (end (line 13) (character 146))) (parent (node (document "d0") (qualified-name "Constraint Assertions-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConstraint") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 26) (character 26)) (end (line 26) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 28) (character 16)) (end (line 28) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 29) (character 20)) (end (line 29) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 32) (character 22)) (end (line 32) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 33) (character 20)) (end (line 33) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConstraint") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (target (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
  )
)
~~~
