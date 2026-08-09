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
# FORMAT
~~~sysml
private import ScalarValues::*;
package AddressBookModel {
    class Entry {
        name: String;
        address: String;
    }

    class AddressBook {
        entries: Entry [*];
    }
}
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
# SMG
~~~
(model
  (namespace
    (namespace_import private -> 'ScalarValues'[unresolved])
    (package 'AddressBookModel'
      (class_def 'Entry'
        (feature_def 'name' : 'String'[unresolved])
        (feature_def 'address' : 'String'[unresolved]))
      (class_def 'AddressBook'
        (feature_def 'entries' : 'AddressBookModel::Entry'[class_def]
          (multiplicity_range [*]))))))
~~~
