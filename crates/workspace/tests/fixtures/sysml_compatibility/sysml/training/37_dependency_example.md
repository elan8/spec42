# META
~~~ini
description=SysML Training 37 (Dependencies): Dependency Example
type=file
~~~
# SOURCE
~~~sysml
package 'Dependency Example' {
	
	part 'System Assembly' {
		part 'Computer Subsystem' {
			// ...
		}
		
		part 'Storage Subsystem' {
			// ...
		}
	}
	
	package 'Software Design' {
		item def MessageSchema {
			// ...
		}
		item def DataSchema {
			// ...
		}
	}
	
	dependency from 'System Assembly'::'Computer Subsystem' to 'Software Design';
	
	dependency Schemata 
		from 'System Assembly'::'Storage Subsystem' 
		to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,UnrestrictedName,OpenCurly,
KwPart,UnrestrictedName,OpenCurly,
LineComment,
CloseCurly,
KwPart,UnrestrictedName,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwItem,KwDef,Ident,OpenCurly,
LineComment,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
KwDependency,KwFrom,UnrestrictedName,ColonColon,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
KwDependency,Ident,
KwFrom,UnrestrictedName,ColonColon,UnrestrictedName,
KwTo,UnrestrictedName,ColonColon,Ident,Comma,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Dependency Example''
    (part_usage ''System Assembly''
      (part_usage ''Computer Subsystem''
        (line_comment))
      (part_usage ''Storage Subsystem''
        (line_comment)))
    (package_def ''Software Design''
      (item_def 'MessageSchema'
        (line_comment))
      (item_def 'DataSchema'
        (line_comment)))
    (dependency from ''System Assembly'::'Computer Subsystem'' to ''Software Design'')
    (dependency 'Schemata' from ''System Assembly'::'Storage Subsystem'' to ''Software Design'::MessageSchema', ''Software Design'::DataSchema')))
~~~
# FORMAT
~~~sysml
package 'Dependency Example' {

    part 'System Assembly' {
        part 'Computer Subsystem' {
            // ...
        }

        part 'Storage Subsystem' {
            // ...
        }
    }

    package 'Software Design' {
        item def MessageSchema {
            // ...
        }
        item def DataSchema {
            // ...
        }
    }

    dependency from 'System Assembly'::'Computer Subsystem' to 'Software Design';

    dependency Schemata
    from 'System Assembly'::'Storage Subsystem'
    to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Dependency Example"))) (name "Dependency Example") (declared-name "Dependency Example")
      (contains
        (element (kind "dependency") (id (node (document "d0") (qualified-name "Dependency Example::Schemata"))) (name "Schemata") (declared-name "Schemata"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Dependency Example::Software Design"))) (name "Software Design") (declared-name "Software Design")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "Dependency Example::Software Design::DataSchema"))) (name "DataSchema") (declared-name "DataSchema"))
            (element (kind "item def") (id (node (document "d0") (qualified-name "Dependency Example::Software Design::MessageSchema"))) (name "MessageSchema") (declared-name "MessageSchema"))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Dependency Example::System Assembly"))) (name "System Assembly") (declared-name "System Assembly") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (name "Computer Subsystem") (declared-name "Computer Subsystem") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (name "Storage Subsystem") (declared-name "Storage Subsystem") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "dependency") (id (node (document "d0") (qualified-name "Dependency Example::dependency"))) (name "dependency") (declared-name "dependency"))
      )
    )
  )
  (relationships
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (to (node (document "d0") (qualified-name "Dependency Example::Software Design"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (to (node (document "d0") (qualified-name "Dependency Example::Software Design::DataSchema"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (to (node (document "d0") (qualified-name "Dependency Example::Software Design::MessageSchema"))))
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
  (document "sysml/training/37_dependency_example.md"
    (diagnostics
    )
  )
)
~~~
