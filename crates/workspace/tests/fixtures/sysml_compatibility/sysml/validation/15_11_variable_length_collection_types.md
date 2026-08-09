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
(model
  (namespace
    (package '15_11-Variable Length Collection Types'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Collections'[unresolved])
      (part_def 'SparePart')
      (part_def 'Person')
      (attribute_def 'Bag<SparePart>' :> 'Bag'[unresolved]
        (part_usage reference :>> 'elements'[unresolved] : '15_11-Variable Length Collection Types::SparePart'[part_def]))
      (attribute_def 'List<Integer>' :> 'List'[unresolved]
        (reference_usage reference 'value' :>> 'elements'[unresolved] : 'Integer'[unresolved]))
      (attribute_def 'Set<String>' :> 'Set'[unresolved]
        (attribute_usage composite :>> 'elements'[unresolved] : 'String'[unresolved]))
      (attribute_def 'OrderedSet<Person>' :> 'OrderedSet'[unresolved]
        (part_usage reference :>> 'elements'[unresolved] : '15_11-Variable Length Collection Types::Person'[part_def]))
      (attribute_def 'List<Set<Person>>' :> 'List'[unresolved]
        (attribute_usage composite :>> 'elements'[unresolved] : 'Set'[unresolved]
          (part_usage reference :>> 'elements'[unresolved] : '15_11-Variable Length Collection Types::Person'[part_def])))
      (attribute_def 'Array<Real>[4]' :> 'Array'[unresolved]
        (attribute_usage composite :>> 'elements'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'dimensions'[unresolved]
          (feature_value (=)))))))
~~~
