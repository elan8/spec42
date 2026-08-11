# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/BaseFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package BaseFunctions {
	doc
	/*
	 * This package defines a basic set of functions defined on all kinds of values. 
	 * Most correspond to similarly named operators in the KerML expression syntax.
	 */

	private import Base::Anything;
	private import Objects::Object;
	private import Metaobjects::Metaobject;
	private import KerML::Metaclass;
	private import ScalarValues::*;
	
	abstract function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x == y);
	}
	
	abstract function '==='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x === y);
	}
	
	abstract function ToString{ in x: Anything[0..1]; 
		return : String;
	}
	
	abstract function '['{ in x: Anything[0..*] nonunique; in y: Anything[0..*] nonunique; 
		return : Anything[0..*] nonunique;
	}
	abstract function '#'{ in seq: Anything[0..*] ordered nonunique; in index: Positive[1..*] ordered nonunique; 
		return : Anything[0..1];
	}
	abstract function ','{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
    abstract function 'all'{
     	return : Object[0..*];
    }
    
	abstract function 'istype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function 'hastype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean;
	}
	
	abstract function '@'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function '@@'{ 
		in seq: Metaobject[0..*];
		in 'type': Metaobject;
		return : Boolean[1];
	}
	
	abstract function 'as'{ 
		in seq: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function 'meta'{ 
		in seq: Metaobject[0..*] ordered nonunique; 
		return : Metaobject[0..*] ordered nonunique;
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Metaobject'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Metaobject'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwNot,OpenParen,Ident,EqEq,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwNot,OpenParen,Ident,EqEqEq,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'BaseFunctions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Objects::Object')
    (import_decl private 'Metaobjects::Metaobject')
    (import_decl private 'KerML::Metaclass')
    (import_decl private 'ScalarValues::*')
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity)
      (feature_def in 'y' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity)
      (feature_def in 'y' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity)
      (feature_def in 'y' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity)
      (feature_def in 'y' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity nonunique)
      (feature_def in 'y' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'index' : 'Positive' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity)
      (feature_def in ''type'' : 'Anything')
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity)
      (feature_def in ''type'' : 'Anything')
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity)
      (feature_def in ''type'' : 'Anything')
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Metaobject' multiplicity)
      (feature_def in ''type'' : 'Metaobject')
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Metaobject' multiplicity ordered nonunique)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package BaseFunctions {
	doc
	/*
	 * This package defines a basic set of functions defined on all kinds of values. 
	 * Most correspond to similarly named operators in the KerML expression syntax.
	 */

	private import Base::Anything;
	private import Objects::Object;
	private import Metaobjects::Metaobject;
	private import KerML::Metaclass;
	private import ScalarValues::*;
	
	abstract function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x == y);
	}
	
	abstract function '==='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1];
	}
	function '!=='{ in x: Anything[0..1]; in y: Anything[0..1]; 
		return : Boolean[1] = not (x === y);
	}
	
	abstract function ToString{ in x: Anything[0..1]; 
		return : String;
	}
	
	abstract function '['{ in x: Anything[0..*] nonunique; in y: Anything[0..*] nonunique; 
		return : Anything[0..*] nonunique;
	}
	abstract function '#'{ in seq: Anything[0..*] ordered nonunique; in index: Positive[1..*] ordered nonunique; 
		return : Anything[0..1];
	}
	abstract function ','{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
    abstract function 'all'{
     	return : Object[0..*];
    }
    
	abstract function 'istype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function 'hastype'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean;
	}
	
	abstract function '@'{ 
		in seq: Anything[0..*];
		in 'type': Anything;
		return : Boolean[1];
	}
	
	abstract function '@@'{ 
		in seq: Metaobject[0..*];
		in 'type': Metaobject;
		return : Boolean[1];
	}
	
	abstract function 'as'{ 
		in seq: Anything[0..*] ordered nonunique; 
		return : Anything[0..*] ordered nonunique;
	}
	
	abstract function 'meta'{ 
		in seq: Metaobject[0..*] ordered nonunique; 
		return : Metaobject[0..*] ordered nonunique;
	}
	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "BaseFunctions"))) (name "BaseFunctions") (declared-name "BaseFunctions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "BaseFunctions::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "BaseFunctions::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "BaseFunctions::Metaclass"))) (name "Metaclass") (declared-name "Metaclass"))
        (element (kind "import") (id (node (document "d0") (qualified-name "BaseFunctions::Metaobject"))) (name "Metaobject") (declared-name "Metaobject"))
        (element (kind "import") (id (node (document "d0") (qualified-name "BaseFunctions::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::ToString"))) (name "ToString") (declared-name "ToString"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "BaseFunctions::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl10"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl11"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl12"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl2"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl3"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl4"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl5"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl6"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl7"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl8"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl9"))) (name "in") (declared-name "in"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BaseFunctions::return"))) (name "return") (declared-name "return"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "BaseFunctions::_documentation"))) (to (node (document "d0") (qualified-name "BaseFunctions"))) (provenance authored))
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
  (document "sysml.library/base_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 28))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 16 1) (end 16 101))
      )
    )
  )
)
~~~
