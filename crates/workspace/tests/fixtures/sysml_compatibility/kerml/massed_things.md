# META
~~~ini
description=KerML Massed Thing: MassedThings
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package MassedThings {
	
	public class MassedThing {
		public name: String;
		public mass: Real = 0;
	}
	
	public assoc MassedThingAssembly {
		public end [0..1] feature assembly: MassedThing;
		public end [0..*] feature parts: MassedThing;
	}
}
~~~
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwClass,Ident,OpenCurly,
KwPublic,Ident,Colon,Ident,Semicolon,
KwPublic,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPublic,KwAssoc,Ident,OpenCurly,
KwPublic,KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
KwPublic,KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'ScalarValues::*')
  (package_def 'MassedThings'
    (class_def public 'MassedThing'
      (feature_def public 'name' : 'String')
      (feature_def public 'mass' : 'Real' value))
    (association_def public 'MassedThingAssembly'
      (feature_def public end 'assembly' multiplicity : 'MassedThing')
      (feature_def public end 'parts' multiplicity : 'MassedThing'))))
~~~
# FORMAT
~~~sysml
private import ScalarValues::*;
package MassedThings {
	
	public class MassedThing {
		public name: String;
		public mass: Real = 0;
	}
	
	public assoc MassedThingAssembly {
		public end [0..1] feature assembly: MassedThing;
		public end [0..*] feature parts: MassedThing;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
    (element (kind "package") (id (node (document "d0") (qualified-name "MassedThings"))) (name "MassedThings") (declared-name "MassedThings")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "MassedThings::MassedThing"))) (name "MassedThing") (declared-name "MassedThing"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "MassedThings::MassedThingAssembly"))) (name "MassedThingAssembly") (declared-name "MassedThingAssembly"))
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
  (document "kerml/massed_things.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 0) (end 0 31))
      )
    )
  )
)
~~~
