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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "37_dependency_example.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "27d13035eb71259c7e40554d57bccac09cebf8f6e9ad722485d1b8ce99ed28f9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Dependency Example"))) (kind "package") (name "Dependency Example") (declared-name "Dependency Example") (range (start (line 0) (character 0)) (end (line 0) (character 488))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::Schemata"))) (kind "dependency") (name "Schemata") (declared-name "Schemata") (range (start (line 23) (character 1)) (end (line 23) (character 138))) (parent (node (document "d0") (qualified-name "Dependency Example"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::Software Design"))) (kind "package") (name "Software Design") (declared-name "Software Design") (range (start (line 12) (character 1)) (end (line 12) (character 110))) (parent (node (document "d0") (qualified-name "Dependency Example"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::Software Design::DataSchema"))) (kind "item def") (name "DataSchema") (declared-name "DataSchema") (range (start (line 16) (character 2)) (end (line 16) (character 37))) (parent (node (document "d0") (qualified-name "Dependency Example::Software Design"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::Software Design::MessageSchema"))) (kind "item def") (name "MessageSchema") (declared-name "MessageSchema") (range (start (line 13) (character 2)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "Dependency Example::Software Design"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::System Assembly"))) (kind "part") (name "System Assembly") (declared-name "System Assembly") (range (start (line 2) (character 1)) (end (line 2) (character 118))) (parent (node (document "d0") (qualified-name "Dependency Example"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (kind "part") (name "Computer Subsystem") (declared-name "Computer Subsystem") (range (start (line 3) (character 2)) (end (line 3) (character 43))) (parent (node (document "d0") (qualified-name "Dependency Example::System Assembly"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (kind "part") (name "Storage Subsystem") (declared-name "Storage Subsystem") (range (start (line 7) (character 2)) (end (line 7) (character 42))) (parent (node (document "d0") (qualified-name "Dependency Example::System Assembly"))))
    (element (id (node (document "d0") (qualified-name "Dependency Example::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 21) (character 1)) (end (line 21) (character 78))) (parent (node (document "d0") (qualified-name "Dependency Example"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
