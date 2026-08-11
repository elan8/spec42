# META
~~~ini
description=KerML Mass Roll-up: MassRollup_2
type=file
~~~
# SOURCE
~~~kerml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Eq,
Ident,Plus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MassRollup_2'
    (import_decl private 'NumericalFunctions::*')
    (import_decl private 'ISQ::*')
    (class_def 'MassedThing'
      (feature_def 'mass' : 'ScalarValues::Real')
      (feature_def 'totalMass' : 'ScalarValues::Real' value)
      (feature_def 'subcomponents' :>> 'massedThings'))
    (feature_def 'massedThings' : 'MassedThing' multiplicity)))
~~~
# FORMAT
~~~sysml
package MassRollup_2 {
	private import NumericalFunctions::*;
	private import ISQ::*;
	
	class MassedThing {
		feature mass : ScalarValues::Real; 
		feature totalMass : ScalarValues::Real =
			mass + sum(subcomponents.totalMass);
			
		feature subcomponents redefines massedThings;	
	}
	
	feature massedThings: MassedThing[0..*];

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MassRollup_2"))) (name "MassRollup_2") (declared-name "MassRollup_2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup_2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MassRollup_2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "MassRollup_2::MassedThing"))) (name "MassedThing") (declared-name "MassedThing"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "MassRollup_2::massedThings"))) (name "massedThings") (declared-name "massedThings"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/mass_rollup_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
    )
  )
)
~~~
