# META
~~~ini
description=KerML Address Book: AddressBookModel
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package AddressBookModel {
	
	class Entry {
		name: String;
		address: String;
	}
	
	class AddressBook {
		entries: Entry[*];
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "address_book_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 27))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'ScalarValues::*')
  (package_def 'AddressBookModel'
    (class_def 'Entry'
      (feature_def 'name' : 'String')
      (feature_def 'address' : 'String'))
    (class_def 'AddressBook'
      (feature_def 'entries' : 'Entry' multiplicity))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# FORMAT
~~~sysml
private import ScalarValues::*;
package AddressBookModel {
	
	class Entry {
		name: String;
		address: String;
	}
	
	class AddressBook {
		entries: Entry[*];
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a2ff6b0a789622e7b61c47aef47962fbd74aae35fa27bc344be488833f10354e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 0) (character 0)) (end (line 0) (character 31))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 0) (character 15)) (end (line 0) (character 27))))))
    (element (id (node (document "d0") (qualified-name "AddressBookModel"))) (kind "package") (name "AddressBookModel") (declared-name "AddressBookModel") (range (start (line 1) (character 0)) (end (line 1) (character 132))))
    (element (id (node (document "d0") (qualified-name "AddressBookModel::AddressBook"))) (kind "classifier decl") (name "AddressBook") (declared-name "AddressBook") (range (start (line 8) (character 1)) (end (line 8) (character 44))) (parent (node (document "d0") (qualified-name "AddressBookModel"))))
    (element (id (node (document "d0") (qualified-name "AddressBookModel::Entry"))) (kind "classifier decl") (name "Entry") (declared-name "Entry") (range (start (line 3) (character 1)) (end (line 3) (character 52))) (parent (node (document "d0") (qualified-name "AddressBookModel"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 0) (character 15)) (end (line 0) (character 27))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
