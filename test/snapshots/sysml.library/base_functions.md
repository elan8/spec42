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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "base_functions.md"
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "58641288fa5434c29e29d198594308c05b97e862e679955a4accdf8c50142ad5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BaseFunctions"))) (kind "package") (name "BaseFunctions") (declared-name "BaseFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 2044))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 1)) (end (line 11) (character 32))) (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 28))))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Metaclass"))) (kind "import") (name "Metaclass") (declared-name "Metaclass") (range (start (line 10) (character 1)) (end (line 10) (character 33))) (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::Metaclass") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 32))))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (range (start (line 9) (character 1)) (end (line 9) (character 40))) (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 39))))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::Object"))) (kind "import") (name "Object") (declared-name "Object") (range (start (line 8) (character 1)) (end (line 8) (character 32))) (parent (node (document "d0") (qualified-name "BaseFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 31))))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::ToString"))) (kind "kermlDecl") (name "ToString") (declared-name "ToString") (range (start (line 27) (character 1)) (end (line 27) (character 73))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2044))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 13) (character 1)) (end (line 13) (character 95))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 16) (character 1)) (end (line 16) (character 101))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl10"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 63) (character 1)) (end (line 63) (character 104))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl11"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 69) (character 1)) (end (line 69) (character 118))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl12"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 74) (character 1)) (end (line 74) (character 124))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl2"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 20) (character 1)) (end (line 20) (character 96))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl3"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 23) (character 1)) (end (line 23) (character 103))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl4"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 31) (character 1)) (end (line 31) (character 128))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl5"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 34) (character 1)) (end (line 34) (character 140))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl6"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 37) (character 1)) (end (line 37) (character 158))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl7"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 45) (character 1)) (end (line 45) (character 104))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl8"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 51) (character 1)) (end (line 51) (character 102))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::in#kermlDecl9"))) (kind "kermlDecl") (name "in") (declared-name "in") (range (start (line 57) (character 1)) (end (line 57) (character 99))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
    (element (id (node (document "d0") (qualified-name "BaseFunctions::return"))) (kind "kermlDecl") (name "return") (declared-name "return") (range (start (line 41) (character 4)) (end (line 41) (character 63))) (parent (node (document "d0") (qualified-name "BaseFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 11) (character 16)) (end (line 11) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Metaclass"))) (kind membershipImport) (ordinal 0)) (authored-target "KerML::Metaclass") (range (start (line 10) (character 16)) (end (line 10) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (range (start (line 9) (character 16)) (end (line 9) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "BaseFunctions::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (range (start (line 8) (character 16)) (end (line 8) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
