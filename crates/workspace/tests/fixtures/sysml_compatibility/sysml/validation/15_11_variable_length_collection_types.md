# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_11-Variable Length Collection Types
type=file
~~~
# SOURCE
~~~sysml
package '15_11-Variable Length Collection Types' {
	private import ScalarValues::*;
	private import Collections::*;
	
	part def SparePart;
	part def Person;
	
	/* Examples of declaring syntactic sugar-like names for instantiating collection types. */
	
	attribute def 'Bag<SparePart>' :> Bag {
		ref part :>> elements: SparePart;
	}
	
	attribute def 'List<Integer>' :> List {
		value :>> elements: Integer;
	}
	
	attribute def 'Set<String>' :> Set {
		attribute :>> elements: String;
	}
	
	attribute def 'OrderedSet<Person>' :> OrderedSet {
		ref part :>> elements: Person;
	}
	
	attribute def 'List<Set<Person>>' :> List {
		attribute :>> elements: Set {
			ref part :>> elements: Person;
		}
	}
	
	attribute def 'Array<Real>[4]' :> Array {
		attribute :>> elements: Real;
		attribute :>> dimensions = 4;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_11-Variable Length Collection Types''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Collections::*')
    (part_def 'SparePart')
    (part_def 'Person')
    (comment)
    (attribute_def ''Bag<SparePart>'' :> 'Bag'
      (part_usage ref :>> 'elements' : 'SparePart'))
    (attribute_def ''List<Integer>'' :> 'List'
      (default_ref_usage 'value' :>> 'elements' : 'Integer'))
    (attribute_def ''Set<String>'' :> 'Set'
      (attribute_usage :>> 'elements' : 'String'))
    (attribute_def ''OrderedSet<Person>'' :> 'OrderedSet'
      (part_usage ref :>> 'elements' : 'Person'))
    (attribute_def ''List<Set<Person>>'' :> 'List'
      (attribute_usage :>> 'elements' : 'Set'
        (part_usage ref :>> 'elements' : 'Person')))
    (attribute_def ''Array<Real>[4]'' :> 'Array'
      (attribute_usage :>> 'elements' : 'Real')
      (attribute_usage :>> 'dimensions' value))))
~~~
# FORMAT
~~~sysml
package '15_11-Variable Length Collection Types' {
    private import ScalarValues::*;
    private import Collections::*;

    part def SparePart;
    part def Person;

    /* Examples of declaring syntactic sugar-like names for instantiating collection types. */

    attribute def 'Bag<SparePart>' :> Bag {
        ref part :>> elements : SparePart;
    }

    attribute def 'List<Integer>' :> List {
        value :>> elements : Integer;
    }

    attribute def 'Set<String>' :> Set {
        attribute :>> elements : String;
    }

    attribute def 'OrderedSet<Person>' :> OrderedSet {
        ref part :>> elements : Person;
    }

    attribute def 'List<Set<Person>>' :> List {
        attribute :>> elements : Set {
            ref part :>> elements : Person;
        }
    }

    attribute def 'Array<Real>[4]' :> Array {
        attribute :>> elements : Real;
        attribute :>> dimensions = 4;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Bag'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'String'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Bag'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'String'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (name "15_11-Variable Length Collection Types") (declared-name "15_11-Variable Length Collection Types")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (name "Array<Real>[4]") (declared-name "Array<Real>[4]") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (name "Bag<SparePart>") (declared-name "Bag<SparePart>") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (name "List<Integer>") (declared-name "List<Integer>") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (name "List<Set<Person>>") (declared-name "List<Set<Person>>") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (name "OrderedSet<Person>") (declared-name "OrderedSet<Person>") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person"))) (name "Person") (declared-name "Person") (declared))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (name "Set<String>") (declared-name "Set<String>") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart"))) (name "SparePart") (declared-name "SparePart") (declared))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (to (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (to (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
