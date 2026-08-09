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
    doc /*
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
(model
  (namespace
    (library_package 'BaseFunctions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Metaobjects::Metaobject'[unresolved])
      (membership_import private -> 'KerML::Metaclass'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (function_def abstract '=='
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '!='
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def abstract '==='
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def '!=='
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def in 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def abstract 'ToString'
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out : 'String'[unresolved])))
      (function_def abstract '['
        (feature_def in 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract '#'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'index' : 'Positive'[unresolved]
          (multiplicity_range [1..*]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1]))))
      (function_def abstract ','
        (feature_def in ordered 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'all'
        (return_parameter_membership
          (feature_def out : 'Object'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'istype'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'type' : 'Anything'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'hastype'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'type' : 'Anything'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved])))
      (function_def abstract '@'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'type' : 'Anything'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '@@'
        (feature_def in 'seq' : 'Metaobject'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'type' : 'Metaobject'[unresolved])
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract 'as'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (function_def abstract 'meta'
        (feature_def in ordered 'seq' : 'Metaobject'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Metaobject'[unresolved]
            (multiplicity_range [0..*])))))))
~~~
