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

    dependency Schemata from 'System Assembly'::'Storage Subsystem' to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
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
(model
  (namespace
    (package 'Dependency Example'
      (part_usage 'System Assembly'
        (part_usage composite 'Computer Subsystem')
        (part_usage composite 'Storage Subsystem'))
      (package 'Software Design'
        (item_def 'MessageSchema')
        (item_def 'DataSchema'))
      (dependency)
      (dependency 'Schemata'))))
~~~
