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
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AddressBookModel"))) (kind "package") (name "AddressBookModel") (declared-name "AddressBookModel"))
    (element (id (node (document "d0") (qualified-name "AddressBookModel::AddressBook"))) (kind "classifier decl") (name "AddressBook") (declared-name "AddressBook") (parent (node (document "d0") (qualified-name "AddressBookModel"))))
    (element (id (node (document "d0") (qualified-name "AddressBookModel::Entry"))) (kind "classifier decl") (name "Entry") (declared-name "Entry") (parent (node (document "d0") (qualified-name "AddressBookModel"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 15) (end 0 27)) (probe (position 0 15))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 0 15) (end 0 27))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
